import { invoke } from '@tauri-apps/api/core'
import { useCategories } from '$lib/category.svelte'
import { usePasswords } from '$lib/password.svelte'
import { useNotes } from '$lib/note.svelte'

let initialized = false
let isUnlocked = $state(false)
let isConfigured: boolean | null = $state(null)

async function doCheckConfigured() {
  if (initialized) return
  initialized = true

  try {
    const configured: boolean = await invoke('is_master_configured')
    isConfigured = configured
  } catch {
    isConfigured = false
  }
}

export function useAuth() {
  if (!initialized) doCheckConfigured()

  const setup = async (password: string) => {
    try {
      await invoke('setup_master_password', { password })
      isUnlocked = true
      isConfigured = true
      return true
    } catch {
      return false
    }
  }

  const unlock = async (password: string) => {
    try {
      await invoke('verify_master_password', { password })
      isUnlocked = true
      return true
    } catch {
      return false
    }
  }

  const changePassword = async (oldPassword: string, newPassword: string) => {
    try {
      await invoke('change_master_password', { oldPassword, newPassword })
      return true
    } catch {
      return false
    }
  }

  const lock = async () => {
    useNotes().wipe()
    usePasswords().wipe()
    useCategories().wipe()

    isUnlocked = false
    try {
      await invoke('lock_session')
    } catch {
      /* best-effort */
    }
  }

  return {
    get isConfigured() {
      return isConfigured
    },
    get isUnlocked() {
      return isUnlocked
    },
    changePassword,
    checkConfigured: doCheckConfigured,
    lock,
    setup,
    unlock,
  }
}
