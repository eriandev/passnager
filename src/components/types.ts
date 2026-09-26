import type { Snippet } from 'svelte'
import type { ButtonRootProps } from 'bits-ui'
import type { HTMLInputAttributes } from 'svelte/elements'

export interface AlertProps {
  open: boolean
  title: string
  description?: string[]
  actions: Array<{
    label: string
    variant?: ButtonProps['variant']
    action?: () => void
  }>
}
export type ButtonProps = ButtonRootProps & {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
}
export interface CardProps<T> {
  entry: T
  onedit: (entry: T) => void
  ondelete: (id: string) => void
}
export interface InputProps extends HTMLInputAttributes {
  leftIcon?: Snippet
  rightIcon?: Snippet
}
export interface ModalAddProps {
  open: boolean
}
export interface ModalEditProps<T> {
  open: boolean
  entry: T | null
}
export interface ModalProps {
  open: boolean
  title?: string
  closeable?: boolean
  description?: string
  children: Snippet
}
export type PasswordInputProps = Exclude<InputProps, 'rightIcon'>
export interface SelectProps {
  id?: string
  value?: string
  items?: {
    value: string
    label: string
    disabled?: boolean
  }[]
  icon?: Snippet
  placeholder?: string
  onValueChange: (v?: string) => void
}
export interface SidebarProps {
  currentPath?: string
}
