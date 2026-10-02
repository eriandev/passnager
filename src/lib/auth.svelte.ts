import { invoke } from '@tauri-apps/api/core'
import { useCategories } from '$lib/category.svelte'
import { usePasswords } from '$lib/password.svelte'
import { useNotes } from '$lib/note.svelte'

let initialized = false
let isUnlocked = $state(false)
let isConfigured: boolean | null = $state(null)
// Set when the check could not be completed. Kept apart from `isConfigured` so a
// failure never turns into a claim: `is_master_configured` used to answer `false`
// on any database error, which is indistinguishable from a first run, so a vault
// whose database could not be read looked like an empty one and its owner was sent
// to `/setup`. Leaving the state unknown is what makes that visible.
let configuredError: string | null = $state(null)

async function doCheckConfigured() {
  if (initialized) return
  initialized = true

  try {
    const configured: boolean = await invoke('is_master_configured')
    isConfigured = configured
    configuredError = null
  } catch (e) {
    // `null` means "not known yet", which is now also what a failure leaves
    // behind. Callers must not read that as `false`.
    isConfigured = null
    configuredError = e instanceof Error ? e.message : String(e)
  }
}

export function useAuth() {
  if (!initialized) doCheckConfigured()

  const setup = async (password: string) => {
    try {
      await invoke('setup_master_password', { password })
      isUnlocked = true
      isConfigured = true
      configuredError = null
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
    // Surfaced so a caller can tell a vault that could not be read from a vault that does not exist.
    get configuredError() {
      return configuredError
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
