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
  categories: EntryCategoryProps[]
  oncopy: (id: string) => Promise<void>
}
export interface CardNoteProps extends CardProps<EntryNoteProps> {
  categories: EntryCategoryProps[]
  oncopy: (id: string) => Promise<void>
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
  categories: EntryCategoryProps[]
  onadd: (data: EntryNoteData & { content: string }) => Promise<void>
}
export interface ModalAddPasswordProps {
  open: boolean
  categories: EntryCategoryProps[]
  onadd: (data: EntryPasswordData & { password: string }) => Promise<void>
}
export interface ModalEditCategoryProps {
  open: boolean
  entry: EntryCategoryProps | null
  onupdate: (id: string, data: EntryCategoryData) => Promise<void>
}
export interface ModalEditNoteProps {
  open: boolean
  entry: EntryNoteProps | null
  categories: EntryCategoryProps[]
  onupdate: (id: string, data: EntryNoteData) => Promise<void>
}
export interface ModalEditPasswordProps {
  open: boolean
  entry: EntryPasswordProps | null
  categories: EntryCategoryProps[]
  onupdate: (id: string, data: EntryPasswordData) => Promise<void>
}
export interface ModalProps {
  open: boolean
  title?: string
  description?: string
  children: Snippet
}
export type PasswordInputProps = Omit<InputProps, 'rightIcon'>
export interface SelectProps {
  id?: string
  /**
   * `null` means "nothing selected". Not bindable on purpose: bits-ui speaks `''`
   * for the same thing, so a write-back from it would put a second empty value
   * straight back into the parent's state. The parent owns the value and this
   * component only reports changes through `onValueChange`.
   */
  value?: string | null
  items?: {
    /**
     * `''` is the one item that means "nothing selected". bits-ui needs a real
     * item to render a label for, so the option exists as a sentinel rather than
     * as something the rest of the app has to keep out of its way.
     */
    value: string
    label: string
    disabled?: boolean
  }[]
  icon?: Snippet
  placeholder?: string
  onValueChange: (v: string | null) => void
}
export interface SidebarProps {
  currentPath?: string
  onlock: () => void
  onnavigate: (path: string) => void
}
export interface TextareaProps extends HTMLTextareaAttributes {
  value?: string
}
