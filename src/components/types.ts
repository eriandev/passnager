import type { Snippet } from 'svelte'
import type { ButtonRootProps } from 'bits-ui'
import type { HTMLInputAttributes } from 'svelte/elements'
import type { EntryPasswordProps } from '$lib/types'

export type ButtonProps = ButtonRootProps & {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
}
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
export interface InputProps extends HTMLInputAttributes {
  leftIcon?: Snippet
  rightIcon?: Snippet
}
export interface ModalAddPassProps {
  open: boolean
}
export interface ModalEditPassProps {
  open: boolean
  entry: EntryPasswordProps | null
}
export interface ModalProps {
  open: boolean
  title?: string
  closeable?: boolean
  description?: string
  children: Snippet
}
export interface PasswordCardProps {
  entry: EntryPasswordProps
  onedit: (entry: EntryPasswordProps) => void
  ondelete: (id: string) => void
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
