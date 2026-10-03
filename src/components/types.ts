import type { ButtonRootProps } from 'bits-ui'
import type { Component, Snippet } from 'svelte'
import type { HTMLInputAttributes, HTMLTextareaAttributes } from 'svelte/elements'
import type {
  EntryCategoryData,
  EntryCategoryProps,
  EntryNoteData,
  EntryNoteProps,
  EntryPasswordData,
  EntryPasswordProps,
} from '$lib/types'

export interface AlertProps {
  open: boolean
  title: string
  description?: string[]
  actions: Array<{
    label: string
    disabled?: boolean
    variant?: ButtonProps['variant']
    action?: () => void
  }>
}
export type ButtonProps = ButtonRootProps & {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost'
}
export interface CardAction {
  title: string
  icon?: Component
  danger?: boolean
  action?: () => void
}
export interface CardProps<T> {
  entry: T
  onedit: (entry: T) => void
  ondelete: (id: string) => void
}
export interface CardPasswordProps extends CardProps<EntryPasswordProps> {
  oncopy: (id: string) => Promise<void>
  categories: EntryCategoryProps[]
}
export interface CardNoteProps extends CardProps<EntryNoteProps> {
  oncopy: (id: string) => Promise<void>
  categories: EntryCategoryProps[]
}
export interface ColorPickerProps {
  id?: string
  value?: string
}
export interface InputProps extends HTMLInputAttributes {
  leftIcon?: Snippet
  rightIcon?: Snippet
}
export interface ModalAddCategoryProps {
  open: boolean
  onadd: (data: EntryCategoryData) => Promise<void>
}
export interface ModalAddNoteProps {
  open: boolean
  onadd: (data: EntryNoteData & { content: string }) => Promise<void>
  categories: EntryCategoryProps[]
}
export interface ModalAddPasswordProps {
  open: boolean
  onadd: (data: EntryPasswordData & { password: string }) => Promise<void>
  categories: EntryCategoryProps[]
}
export interface ModalEditCategoryProps {
  open: boolean
  entry: EntryCategoryProps | null
  onupdate: (id: string, data: EntryCategoryData) => Promise<void>
}
export interface ModalEditNoteProps {
  open: boolean
  entry: EntryNoteProps | null
  onupdate: (id: string, data: EntryNoteData) => Promise<void>
  categories: EntryCategoryProps[]
}
export interface ModalEditPasswordProps {
  open: boolean
  entry: EntryPasswordProps | null
  onupdate: (id: string, data: EntryPasswordData) => Promise<void>
  categories: EntryCategoryProps[]
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
  onlock: () => void
  onnavigate: (path: string) => void
}
export interface TextareaProps extends HTMLTextareaAttributes {
  value?: string
}
