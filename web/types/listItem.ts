export type LinkItem = {
  title: string
  to: string
  icon?: string
  external?: boolean
  login?: boolean
  logout?: boolean
  ty: 'link'
}

export type FuncItem = {
  title: string
  icon?: string
  login?: boolean
  logout?: boolean
  action: () => void
  ty: 'func'
}

export type Item = LinkItem | FuncItem
