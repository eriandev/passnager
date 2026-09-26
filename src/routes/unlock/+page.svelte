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

  async function handleSubmit(e: Event) {
    e.preventDefault()
    error = ''

    if (!password) {
      error = 'Enter the master password'
      return
    }

    loading = true
    const success = await auth.unlock(password)
    loading = false

    if (success) {
      toast.success('Vault unlocked')
      navigate.goto('/(app)/passwords')
    } else {
      error = 'Incorrect master password'
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
        <hgroup class="text-center">
          <h1 class="text-2xl font-bold text-foreground">Passnager</h1>
          <p class="mt-1 text-sm text-foreground-alt">Enter your master password to unlock</p>
        </hgroup>
      </header>

      <section class="grid gap-y-4">
        <PasswordInput
          id="password"
          autocomplete="off"
          bind:value={password}
          placeholder="Master password"
          disabled={loading}
        />

        <span class={['min-h-4 text-xs font-bold text-destructive', { invisible: !error }]}>
          {error}
        </span>

        <Button type="submit" disabled={loading} class="w-full">
          {loading ? 'Unlocking...' : 'Unlock'}
        </Button>
      </section>
    </form>
  </article>
</main>
