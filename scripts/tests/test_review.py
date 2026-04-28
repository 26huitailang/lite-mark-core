#!/usr/bin/env python3
"""review.py 单元测试"""

import io
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import MagicMock, patch

sys.path.insert(0, str(Path(__file__).parent.parent))

import review


class TestLoadSystemPrompt(unittest.TestCase):
    def setUp(self):
        self.root = Path(review.__file__).parent.parent.resolve()

    def test_load_valid_file(self):
        path = self.root / ".kimi" / "agents" / "reviewer-system.md"
        content = review.load_system_prompt(str(path))
        self.assertIn("角色定义", content)

    def test_path_traversal_rejected(self):
        with self.assertRaises(ValueError) as ctx:
            review.load_system_prompt("/etc/passwd")
        self.assertIn("project root", str(ctx.exception))

    def test_nonexistent_file(self):
        with self.assertRaises(ValueError) as ctx:
            review.load_system_prompt(str(self.root / "nonexistent.md"))
        self.assertIn("valid file", str(ctx.exception))

    def test_directory_rejected(self):
        with self.assertRaises(ValueError) as ctx:
            review.load_system_prompt(str(self.root / ".kimi"))
        self.assertIn("valid file", str(ctx.exception))

    def test_oversized_file(self):
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".md", dir=str(self.root), delete=False
        ) as f:
            f.write("x" * (review.MAX_PROMPT_SIZE_BYTES + 1))
            tmp = f.name
        try:
            with self.assertRaises(ValueError) as ctx:
                review.load_system_prompt(tmp)
            self.assertIn("too large", str(ctx.exception))
        finally:
            os.unlink(tmp)


class TestCallMoonshotApi(unittest.TestCase):
    def _mock_response(self, content=None, reasoning_content=None):
        resp = MagicMock()
        payload = {
            "id": "test",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": content,
                        "reasoning_content": reasoning_content,
                    },
                }
            ],
        }
        resp.read.return_value = json.dumps(payload).encode()
        return resp

    @patch("urllib.request.urlopen")
    def test_success(self, mock_urlopen):
        mock_urlopen.return_value.__enter__.return_value = self._mock_response(
            content='{"result": "ok"}'
        )
        result = review.call_moonshot_api("sys", "user", "fake-key")
        self.assertEqual(result, '{"result": "ok"}')

    @patch("urllib.request.urlopen")
    def test_reasoning_fallback(self, mock_urlopen):
        mock_urlopen.return_value.__enter__.return_value = self._mock_response(
            content=None, reasoning_content='{"result": "from reasoning"}'
        )
        result = review.call_moonshot_api("sys", "user", "fake-key")
        self.assertEqual(result, '{"result": "from reasoning"}')

    @patch("urllib.request.urlopen")
    def test_empty_string_uses_reasoning(self, mock_urlopen):
        mock_urlopen.return_value.__enter__.return_value = self._mock_response(
            content="", reasoning_content='{"result": "fallback"}'
        )
        result = review.call_moonshot_api("sys", "user", "fake-key")
        self.assertEqual(result, '{"result": "fallback"}')

    @patch("urllib.request.urlopen")
    def test_empty_content_no_reasoning(self, mock_urlopen):
        mock_urlopen.return_value.__enter__.return_value = self._mock_response(
            content=None, reasoning_content=""
        )
        with self.assertRaises(RuntimeError) as ctx:
            review.call_moonshot_api("sys", "user", "fake-key")
        self.assertIn("empty content", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_http_401(self, mock_urlopen):
        from urllib.error import HTTPError

        fp = io.BytesIO(b'{"error": "unauthorized"}')
        mock_urlopen.side_effect = HTTPError(
            "https://api.moonshot.cn/v1/chat/completions",
            401,
            "Unauthorized",
            {},
            fp,
        )
        with self.assertRaises(RuntimeError) as ctx:
            review.call_moonshot_api("sys", "user", "fake-key")
        self.assertIn("authentication failed", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_http_429(self, mock_urlopen):
        from urllib.error import HTTPError

        fp = io.BytesIO(b'{"error": "rate limited"}')
        mock_urlopen.side_effect = HTTPError(
            "https://api.moonshot.cn/v1/chat/completions",
            429,
            "Too Many Requests",
            {},
            fp,
        )
        with self.assertRaises(RuntimeError) as ctx:
            review.call_moonshot_api("sys", "user", "fake-key")
        self.assertIn("rate limited", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_http_500(self, mock_urlopen):
        from urllib.error import HTTPError

        fp = io.BytesIO(b'{"error": "server error"}')
        mock_urlopen.side_effect = HTTPError(
            "https://api.moonshot.cn/v1/chat/completions",
            500,
            "Internal Server Error",
            {},
            fp,
        )
        with self.assertRaises(RuntimeError) as ctx:
            review.call_moonshot_api("sys", "user", "fake-key")
        self.assertIn("server error", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_no_choices(self, mock_urlopen):
        resp = MagicMock()
        resp.read.return_value = json.dumps({"choices": []}).encode()
        mock_urlopen.return_value.__enter__.return_value = resp
        with self.assertRaises(RuntimeError) as ctx:
            review.call_moonshot_api("sys", "user", "fake-key")
        self.assertIn("no choices", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_invalid_json(self, mock_urlopen):
        resp = MagicMock()
        resp.read.return_value = b"not json"
        mock_urlopen.return_value.__enter__.return_value = resp
        with self.assertRaises(RuntimeError) as ctx:
            review.call_moonshot_api("sys", "user", "fake-key")
        self.assertIn("invalid JSON", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_url_error(self, mock_urlopen):
        from urllib.error import URLError

        mock_urlopen.side_effect = URLError("connection refused")
        with self.assertRaises(RuntimeError) as ctx:
            review.call_moonshot_api("sys", "user", "fake-key")
        self.assertIn("connection error", str(ctx.exception))


class TestExtractJson(unittest.TestCase):
    def test_direct_json(self):
        data = {"summary": "ok", "issues": []}
        result = review.extract_json(json.dumps(data))
        self.assertEqual(result["summary"], "ok")

    def test_json_in_code_block(self):
        text = 'Some text\n```json\n{"summary": "ok", "issues": []}\n```\nMore text'
        result = review.extract_json(text)
        self.assertEqual(result["summary"], "ok")

    def test_json_in_braces(self):
        text = 'Prefix {"summary": "ok", "issues": []} suffix'
        result = review.extract_json(text)
        self.assertEqual(result["summary"], "ok")

    def test_no_json(self):
        result = review.extract_json("just plain text")
        self.assertEqual(result["summary"], "解析审查结果失败")


class TestTruncateDiff(unittest.TestCase):
    def test_no_truncation(self):
        diff = "line1\nline2\nline3"
        result = review.truncate_diff(diff, 10)
        self.assertEqual(result, diff)

    def test_truncation(self):
        diff = "\n".join([f"line{i}" for i in range(100)])
        result = review.truncate_diff(diff, 50)
        self.assertIn("diff 过长，已截断", result)
        self.assertLess(len(result.splitlines()), 60)


class TestParseStat(unittest.TestCase):
    def test_normal(self):
        stat = "src/foo.rs | 10 ++++++-----\n 2 files changed, 12 insertions(+), 5 deletions(-)"
        result = review.parse_stat(stat)
        self.assertEqual(result["files_changed"], 2)
        self.assertEqual(result["lines_added"], 12)
        self.assertEqual(result["lines_removed"], 5)

    def test_empty(self):
        result = review.parse_stat("")
        self.assertEqual(result["files_changed"], 0)


if __name__ == "__main__":
    unittest.main()
