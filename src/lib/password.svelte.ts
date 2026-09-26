import { invoke } from '@tauri-apps/api/core'
import type { EntryPasswordData, EntryPasswordProps } from '$lib/types'

let loading = $state(true)
let list = $state<EntryPasswordProps[]>([])

export function usePasswords() {
  const load = async (categoryId?: string) => {
    loading = true
    try {
      list = await invoke<EntryPasswordProps[]>('get_passwords', { categoryId })
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
      categoryId: data.categoryId ?? null,
    })
    list = [entry, ...list]
  }

  const update = async (id: string, data: EntryPasswordData) => {
    await invoke('update_password', {
      id,
      url: data.url,
      username: data.username,
      password: data.password || null,
      categoryId: data.categoryId ?? null,
    })
    list = list.map((entry) =>
      entry.id === id
        ? {
            ...entry,
            url: data.url,
            username: data.username,
            categoryId: data.categoryId ?? null,
            updatedAt: String(Math.floor(Date.now() / 1000)),
          }
        : entry,
    )
  }

  const remove = async (id: string) => {
    await invoke('delete_password', { id })
    list = list.filter((entry) => entry.id !== id)
  }

  const decrypt = async (id: string) => {
    return await invoke<string>('decrypt_password_by_id', { id })
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
  }
}
