export type Anchor =
  | 'top-left'
  | 'top-right'
  | 'bottom-left'
  | 'bottom-right'
  | 'bottom-center'
  | 'center'

export type RenderMode = 'bottom-frame' | 'gradient-frame' | 'overlay' | 'minimal'

export type ItemType = 'text' | 'logo'

export type FontWeight = 'normal' | 'bold' | 'light'

export type EditorMode = 'preset' | 'visual' | 'json'

export interface TemplateItem {
  type: ItemType
  value: string
  font_size_ratio: number
  weight: FontWeight | null
  color: string | null
}

export interface Background {
  type: 'rect' | 'gradient' | 'none'
  color?: string
  radius?: number
  padding?: number
  direction?: 'top-to-bottom' | 'left-to-right'
  start_color?: string
  end_color?: string
  start_opacity?: number
  end_opacity?: number
}

export interface TemplateConfig {
  name: string
  anchor: Anchor
  padding: number
  frame_height_ratio: number
  logo_size_ratio: number
  primary_font_ratio: number
  secondary_font_ratio: number
  padding_ratio: number
  render_mode: RenderMode
  items: TemplateItem[]
  background: Background | null
  line_spacing_ratio?: number
}
