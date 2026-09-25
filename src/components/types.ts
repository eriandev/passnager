import type { Snippet } from 'svelte'
import type { ButtonRootProps } from 'bits-ui'
import type { HTMLInputAttributes } from 'svelte/elements'

export type ButtonProps = ButtonRootProps & {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
}
export interface InputProps extends HTMLInputAttributes {
  leftIcon?: Snippet<[]>
  rightIcon?: Snippet<[]>
}
export type PasswordInputProps = Exclude<InputProps, 'rightIcon'>
export interface SidebarProps {
  currentPath?: string
}
