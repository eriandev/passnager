<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { AlertDialog } from 'bits-ui'
  import Input from '@/components/input.svelte'
  import Modal from '@/components/modal.svelte'
  import { CATEGORY_DEFAULT_COLOR, CATEGORY_ICONS } from '$lib/consts'
  import ColorPicker from '@/components/color-picker.svelte'
  import type { ModalAddCategoryProps } from '@/components/types'

  let { open = $bindable(false), onadd }: ModalAddCategoryProps = $props()

  let formName = $state('')
  let formError = $state('')
  let submitting = $state(false)
  let formIcon = $state<string | null>(null)
  let formColor = $state(CATEGORY_DEFAULT_COLOR)

  $effect(() => {
    if (open) {
      formName = ''
      formError = ''
      formIcon = null
      formColor = CATEGORY_DEFAULT_COLOR
    }
  })

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault()

    if (submitting) return
    formError = ''

    if (!formName.trim()) {
      formError = 'Name is required'
      return
    }

    submitting = true
    try {
      await onadd({ name: formName.trim(), icon: formIcon, color: formColor })
      toast.success('Category created')
      open = false
    } catch (err) {
      formError = String(err)
    } finally {
      submitting = false
    }
  }
</script>

<Modal bind:open title="New category">
  <form onsubmit={handleSubmit} class="grid gap-y-4">
    <section class="flex flex-col gap-y-4 text-sm font-medium">
      <div class="grid gap-y-1">
        <label for="category-name">Name</label>
        <Input type="text" id="category-name" bind:value={formName} placeholder="My category" />
      </div>

      <div class="grid gap-y-1">
        <span id="category-icon-label">Icon</span>
        <div role="group" aria-labelledby="category-icon-label" class="flex flex-wrap gap-2">
          {#each CATEGORY_ICONS as icon, i (icon + i)}
            <button
              type="button"
              class={[
                'flex size-10 items-center justify-center rounded-lg text-lg transition-colors',
                formIcon === icon ? 'bg-primary/15 ring-2 ring-primary' : 'bg-dark-10 hover:bg-dark-40',
              ]}
              onclick={() => (formIcon = icon)}
            >
              {icon}
            </button>
          {/each}
        </div>
      </div>

      <div class="grid gap-y-1">
        <label for="category-color">Color</label>
        <ColorPicker id="category-color" bind:value={formColor} />
      </div>
    </section>

    <span class={['min-h-4 text-xs font-bold text-destructive', { invisible: !formError }]}>
      {formError}
    </span>

    <section class="flex justify-end gap-x-3">
      <AlertDialog.Cancel type="button" class="btn secondary" disabled={submitting}>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action type="submit" class="btn primary" disabled={submitting}>
        {submitting ? 'Adding...' : 'Create category'}
      </AlertDialog.Action>
    </section>
  </form>
</Modal>
