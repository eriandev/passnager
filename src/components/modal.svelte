<script lang="ts">
  import { AlertDialog, Separator } from 'bits-ui'
  import type { ModalProps } from '@/components/types'

  let { open = $bindable(false), title = '', description, children }: ModalProps = $props()
</script>

<AlertDialog.Root bind:open>
  <AlertDialog.Portal>
    <AlertDialog.Overlay
      class="fixed inset-0 z-50 bg-black/80 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:animate-in data-[state=open]:fade-in-0"
    />
    <AlertDialog.Content
      class="fixed top-[50%] left-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] rounded-card-lg border bg-background p-5 shadow-popover outline-hidden data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 sm:max-w-122.5 md:w-full md:max-w-128.5"
    >
      <AlertDialog.Title class="flex w-full items-center justify-center text-lg font-semibold tracking-tight">
        {title}
      </AlertDialog.Title>

      <Separator.Root class="-mx-5 my-6 block h-px bg-muted" />

      {#if description}
        <AlertDialog.Description class="text-sm text-foreground-alt">
          {description}
        </AlertDialog.Description>
      {/if}

      {@render children?.()}
    </AlertDialog.Content>
  </AlertDialog.Portal>
</AlertDialog.Root>
