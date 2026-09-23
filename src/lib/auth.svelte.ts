import { invoke } from '@tauri-apps/api/core'

let dek = $state('')
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
      const dekBase64: string = await invoke('setup_master_password', { password })
      isUnlocked = true
      isConfigured = true
      dek = dekBase64
      return true
    } catch {
      return false
    }
  }

  const unlock = async (password: string) => {
    try {
      const dekBase64: string = await invoke('verify_master_password', { password })
      isUnlocked = true
      dek = dekBase64
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

  const lock = () => {
    isUnlocked = false
    dek = ''
  }

  return {
    get isConfigured() {
      return isConfigured
    },
    get isUnlocked() {
      return isUnlocked
    },
    get dek() {
      return dek
    },
    changePassword,
    checkConfigured: doCheckConfigured,
    lock,
    setup,
    unlock,
  }
}
