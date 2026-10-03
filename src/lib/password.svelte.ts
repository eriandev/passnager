import { invoke } from '@tauri-apps/api/core'
import type { EntryPasswordData, EntryPasswordProps } from '$lib/types'

let loading = $state(true)
let list = $state<EntryPasswordProps[]>([])

// Which slice of the vault `list` currently holds. `load` replaces the list with a
// single category's rows, so `add` and `update` have to respect that filter:
// prepending blindly dropped a "Personal" password at the top of the "Work" filter,
// where it stayed until the user navigated away and back.
let loadedCategoryId: string | null

const inList = (entry: EntryPasswordProps) => loadedCategoryId === null || entry.categoryId === loadedCategoryId

export function usePasswords() {
  const load = async (categoryId: string | null) => {
    loading = true
    try {
      list = await invoke<EntryPasswordProps[]>('get_passwords', { categoryId })
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

  const add = async (data: EntryPasswordData & { password: string }) => {
    const entry = await invoke<EntryPasswordProps>('add_password', {
      url: data.url,
      username: data.username,
      password: data.password,
      categoryId: data.categoryId,
    })

    if (inList(entry)) list = [entry, ...list]
  }

  const update = async (id: string, data: EntryPasswordData) => {
    await invoke('update_password', {
      id,
      url: data.url,
      username: data.username,
      categoryId: data.categoryId,
      password: data.password || null,
    })
    // Relabelling in place is not enough: an edit that moves the entry out of the
    // category being viewed has to take it off the list too.
    list = list
      .map((entry) =>
        entry.id === id
          ? {
              ...entry,
              url: data.url,
              username: data.username,
              categoryId: data.categoryId,
              updatedAt: String(Math.floor(Date.now() / 1000)),
            }
          : entry,
      )
      .filter((entry) => entry.id !== id || inList(entry))
  }

  const remove = async (id: string) => {
    await invoke('delete_password', { id })
    list = list.filter((entry) => entry.id !== id)
  }

  const decrypt = async (id: string) => {
    return await invoke<string>('decrypt_password_by_id', { id })
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
    load,
    decrypt,
    remove,
    update,
    wipe,
  }
}
