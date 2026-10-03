import { invoke } from '@tauri-apps/api/core'
import type { EntryCategoryData, EntryCategoryProps } from '$lib/types'

let loading = $state(true)
let list = $state<EntryCategoryProps[]>([])

const byName = (a: EntryCategoryProps, b: EntryCategoryProps) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0)

export function categoryLabel(category: Pick<EntryCategoryProps, 'icon' | 'name'>): string {
  return category.icon ? `${category.icon} ${category.name}` : category.name
}

export function useCategories() {
  const load = async () => {
    loading = true
    try {
      list = await invoke<EntryCategoryProps[]>('get_categories')
      return true
    } catch {
      return false
    } finally {
      loading = false
    }
  }

  const add = async (data: EntryCategoryData) => {
    const entry = await invoke<EntryCategoryProps>('add_category', {
      name: data.name,
      icon: data.icon,
      color: data.color,
    })
    list = [...list, entry].sort(byName)
  }

  const update = async (id: string, data: EntryCategoryData) => {
    await invoke('update_category', {
      id,
      name: data.name,
      icon: data.icon,
      color: data.color,
    })
    list = list
      .map((entry) => (entry.id === id ? { ...entry, name: data.name, icon: data.icon, color: data.color } : entry))
      .sort(byName)
  }

  const remove = async (id: string) => {
    await invoke('delete_category', { id })
    list = list.filter((entry) => entry.id !== id)
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
    remove,
    update,
    wipe,
  }
}
