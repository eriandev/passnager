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

// `initialized` used to be latched *before* the await, so a check that failed — or
// was still running when a redirect read it — never ran again and the app stayed
// wrong until it was restarted. It now latches on success only, and a failure backs
// off and tries again: a transient SQLite lock clears on its own, so retrying is
// what turns "restart the app" into "wait a moment".
const RETRY_BASE_MS = 500
const RETRY_MAX_MS = 10_000
let retryMs = RETRY_BASE_MS
let retryTimer: ReturnType<typeof setTimeout> | undefined

function scheduleRetry() {
  clearTimeout(retryTimer)
  retryTimer = setTimeout(() => void doCheckConfigured(), retryMs)
  retryMs = Math.min(retryMs * 2, RETRY_MAX_MS)
}

async function doCheckConfigured() {
  if (initialized) return

  try {
    const configured: boolean = await invoke('is_master_configured')
    isConfigured = configured
    configuredError = null
    retryMs = RETRY_BASE_MS
    initialized = true
  } catch (e) {
    // `null` means "not known yet", which is now also what a failure leaves
    // behind. Callers must not read that as `false`.
    isConfigured = null
    configuredError = e instanceof Error ? e.message : String(e)
    scheduleRetry()
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

  // Asks for the check again right away, for someone who would rather not wait
  // out the backoff. A no-op once a check has succeeded.
  const retryCheck = async () => {
    clearTimeout(retryTimer)
    retryMs = RETRY_BASE_MS
    await doCheckConfigured()
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
    retryCheck,
    setup,
    unlock,
  }
}
