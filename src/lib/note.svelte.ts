import { invoke } from '@tauri-apps/api/core'
import type { EntryNoteData, EntryNoteProps } from '$lib/types'

let loading = $state(true)
let list = $state<EntryNoteProps[]>([])

// Which slice of the vault `list` currently holds. `load` replaces the list with a
// single category's rows, so `add` and `update` have to respect that filter:
// prepending blindly dropped a "Personal" note at the top of the "Work" filter,
// where it stayed until the user navigated away and back.
let loadedCategoryId: string | null

const inList = (entry: EntryNoteProps) => loadedCategoryId === null || entry.categoryId === loadedCategoryId

export function useNotes() {
  const load = async (categoryId: string | null) => {
    loading = true
    try {
      list = await invoke<EntryNoteProps[]>('get_notes', { categoryId })
      // Only where the list actually changed: on failure it keeps its old contents,
      // so the filter those contents were built for has to survive as well.
      loadedCategoryId = categoryId
      return true
    } catch {
      return false
    } finally {
      loading = false
    }
  }

  const add = async (data: EntryNoteData & { content: string }) => {
    const entry = await invoke<EntryNoteProps>('add_note', {
      color: data.color,
      title: data.title,
      content: data.content,
      categoryId: data.categoryId,
    })

    if (inList(entry)) list = [entry, ...list]
  }

  const update = async (id: string, data: EntryNoteData) => {
    await invoke('update_note', {
      id,
      color: data.color,
      title: data.title,
      content: data.content,
      categoryId: data.categoryId,
    })
    // Relabelling in place is not enough: an edit that moves the entry out of the
    // category being viewed has to take it off the list too.
    list = list
      .map((entry) =>
        entry.id === id
          ? {
              ...entry,
              color: data.color,
              title: data.title,
              categoryId: data.categoryId,
              updatedAt: String(Math.floor(Date.now() / 1000)),
            }
          : entry,
      )
      .filter((entry) => entry.id !== id || inList(entry))
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
