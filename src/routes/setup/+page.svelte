<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { useAuth } from '$lib/auth.svelte'
  import Button from '@/components/button.svelte'
  import LockKey from 'phosphor-svelte/lib/LockKeyIcon'
  import { useNavigation } from '$lib/navigation.svelte'
  import PasswordInput from '@/components/password-input.svelte'

  const auth = useAuth()
  const navigate = useNavigation()

  let error = $state('')
  let password = $state('')
  let loading = $state(false)
  let confirmPassword = $state('')

  async function handleSubmit(e: Event) {
    e.preventDefault()
    error = ''

    if (password.length < 8) {
      error = 'Password must be at least 8 characters'
      return
    }

    if (password !== confirmPassword) {
      error = 'Passwords do not match'
      return
    }

    loading = true
    const success = await auth.setup(password)
    loading = false

    if (success) {
      toast.success('Master password configured')
      navigate.goto('/(app)/passwords')
    } else {
      error = 'Failed to configure master password'
    }
  }
</script>

<main class="grid min-h-screen w-full place-items-center px-4">
  <article
    class="flex w-full max-w-md flex-col gap-4 rounded-xl border border-dark-10 bg-background-alt px-2 py-4 shadow-card"
  >
    <form class="p-4 grid gap-y-8" onsubmit={handleSubmit}>
      <header class="flex flex-col items-center gap-3">
        <div
          class="flex size-16 items-center justify-center rounded-2xl bg-linear-to-br from-primary to-secondary text-contrast"
        >
          <LockKey class="size-8" />
        </div>
        <hgroup class="text-center grid gap-y-1">
          <h1 class="text-2xl font-bold text-foreground">Passnager</h1>
          <p class="text-sm text-foreground-alt">Set up your master password to get started</p>
        </hgroup>
      </header>

      <section class="grid gap-y-4">
        <div class="flex flex-col items-start gap-y-3">
          <PasswordInput
            id="new-password"
            autocomplete="off"
            bind:value={password}
            placeholder="At least 8 characters"
            disabled={loading}
          />

          <PasswordInput
            autocomplete="off"
            id="confirm-new-password"
            bind:value={confirmPassword}
            placeholder="Repeat the password"
            disabled={loading}
          />
        </div>

        <span class={['min-h-4 text-xs font-bold text-destructive', { invisible: !error }]}>
          {error}
        </span>

        <Button type="submit" disabled={loading} class="w-full">
          {loading ? 'Setting up...' : 'Set master password'}
        </Button>
      </section>
    </form>
  </article>
</main>
