import { invoke } from '@tauri-apps/api/core'
import type { EntryNoteData, EntryNoteProps } from '$lib/types'

let loading = $state(true)
let list = $state<EntryNoteProps[]>([])

export function useNotes() {
  const load = async (categoryId?: string) => {
    loading = true
    try {
      list = await invoke<EntryNoteProps[]>('get_notes', { categoryId })
      return true
    } catch {
      return false
    } finally {
      loading = false
    }
  }

  const add = async (data: EntryNoteData & { content: string }) => {
    const entry = await invoke<EntryNoteProps>('add_note', {
      title: data.title,
      content: data.content,
      color: data.color ?? null,
      categoryId: data.categoryId ?? null,
    })
    list = [entry, ...list]
  }

  const update = async (id: string, data: EntryNoteData) => {
    await invoke('update_note', {
      id,
      title: data.title,
      content: data.content ?? null,
      color: data.color ?? null,
      categoryId: data.categoryId ?? null,
    })
    list = list.map((entry) =>
      entry.id === id
        ? {
            ...entry,
            title: data.title,
            color: data.color ?? null,
            categoryId: data.categoryId ?? null,
            updatedAt: String(Math.floor(Date.now() / 1000)),
          }
        : entry,
    )
  }

  const remove = async (id: string) => {
    await invoke('delete_note', { id })
    list = list.filter((entry) => entry.id !== id)
  }

  const content = async (id: string) => {
    return await invoke<string>('decrypt_note_by_id', { id })
  }

  const wipe = () => {
    list = []
  }

  return {
    get list() {
      return list
    },
    get loading() {
      return loading
    },
    add,
    content,
    load,
    remove,
    update,
    wipe,
  }
}
