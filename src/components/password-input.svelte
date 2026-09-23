<script lang="ts">
  import { Toggle } from 'bits-ui'
  import Input from '@/components/input.svelte'
  import LockKey from 'phosphor-svelte/lib/LockKeyIcon'
  import LockKeyOpen from 'phosphor-svelte/lib/LockKeyOpenIcon'
  import type { PasswordInputProps } from '@/components/types'

  let { value = $bindable(), class: extraClass, ...restProps }: PasswordInputProps = $props()

  let isUnlocked = $state(false)
</script>

<Input bind:value {...restProps} type={isUnlocked ? 'text' : 'password'} class={['pr-1!', extraClass]}>
  {#snippet rightIcon()}
    <Toggle.Root
      aria-label="toggle password visibility"
      class="flex cursor-pointer size-10 shrink-0 items-center justify-center hover:bg-muted active:bg-dark-10 data-[state=on]:bg-muted data-[state=off]:text-foreground-alt data-[state=on]:text-foreground active:data-[state=on]:bg-dark-10 rounded-card-sm transition-all active:scale-[0.98]"
      bind:pressed={isUnlocked}
    >
      {#if isUnlocked}
        <LockKeyOpen class="size-6" />
      {:else}
        <LockKey class="size-6 text-muted-foreground" />
      {/if}
    </Toggle.Root>
  {/snippet}
</Input>
