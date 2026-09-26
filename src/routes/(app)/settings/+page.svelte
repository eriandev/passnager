<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/core'

  let appName = $state('')
  let appVersion = $state('')

  onMount(() => {
    invoke<string>('get_app_version').then((v) => {
      appVersion = v
    })
    invoke<string>('get_app_name').then((n) => {
      appName = n
    })
  })
</script>

<div class="grid gap-y-6 mx-auto max-w-4xl">
  <h1 class="text-3xl font-bold text-foreground">Settings</h1>

  <section>
    <article class="grid gap-y-4 rounded-xl border border-border-card bg-background-alt p-6">
      <h2 class="text-lg font-semibold text-foreground">About</h2>
      <div class="flex flex-col gap-2 text-sm text-foreground-alt">
        <p>
          <strong class="text-foreground capitalize">{appName}</strong> v{appVersion}
        </p>
        <p>Desktop password manager</p>
        <p class="text-xs text-muted-foreground">
          Passwords are encrypted locally with Argon2id + AES-256-GCM. Nothing is sent to the Internet.
        </p>
      </div>
    </article>
  </section>
</div>
