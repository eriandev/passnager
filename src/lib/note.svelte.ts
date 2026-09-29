import { invoke } from '@tauri-apps/api/core'
import type { EntryNoteData, EntryNoteProps } from '$lib/types'

let loading = $state(true)
let list = $state<EntryNoteProps[]>([])
let contents = $state<Record<string, string>>({})

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
    contents[entry.id] = data.content
  }

  const update = async (id: string, data: EntryNoteData) => {
    await invoke('update_note', {
      id,
      title: data.title,
      content: data.content ?? null,
      color: data.color ?? null,
      categoryId: data.categoryId ?? null,
    })
    if (data.content) contents[id] = data.content
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
    delete contents[id]
  }

  const content = async (id: string) => {
    const cached = contents[id]
    if (cached !== undefined) return cached

    const plain = await invoke<string>('decrypt_note_by_id', { id })
    contents[id] = plain
    return plain
  }

  const wipe = () => {
    list = []
    contents = {}
  }

  return {
    get list() {
      return list
    },
    get loading() {
      return loading
    },
    get contents() {
      return contents
    },
    add,
    content,
    load,
    remove,
    update,
    wipe,
  }
}
