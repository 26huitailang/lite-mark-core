import type { TemplateConfig } from '@/types'

import classicJson from '@/assets/presets/classic.json'
import compactJson from '@/assets/presets/compact.json'
import professionalJson from '@/assets/presets/professional.json'
import overlayJson from '@/assets/presets/overlay.json'

export const PRESETS: Record<string, TemplateConfig> = {
  classic: classicJson as TemplateConfig,
  compact: compactJson as TemplateConfig,
  professional: professionalJson as TemplateConfig,
  overlay: overlayJson as TemplateConfig,
}

export const PRESET_IDS = Object.keys(PRESETS)

export const PRESET_LABELS: Record<string, string> = {
  classic: 'Classic — 经典底框',
  compact: 'Compact — 极简单行',
  professional: 'Professional — 渐变专业',
  overlay: 'Overlay — 内嵌签名',
}
