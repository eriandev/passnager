<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { AlertDialog } from 'bits-ui'
  import Input from '@/components/input.svelte'
  import Modal from '@/components/modal.svelte'
  import { CATEGORY_DEFAULT_COLOR } from '$lib/consts'
  import ColorPicker from '@/components/color-picker.svelte'
  import type { ModalEditCategoryProps } from '@/components/types'

  const defaultIcons = ['📁', '🔑', '📧', '💳', '🌐', '📱', '💻', '🔒'] as const

  let { open = $bindable(false), entry = null, onupdate }: ModalEditCategoryProps = $props()

  let formName = $state('')
  let formIcon = $state('')
  let formError = $state('')
  let formColor = $state('')
  let submitting = $state(false)
  let formCategoryId = $state('')

  $effect(() => {
    if (open && entry) {
      formError = ''
      formName = entry.name
      formCategoryId = entry.id
      formIcon = entry.icon || ''
      formColor = entry.color || CATEGORY_DEFAULT_COLOR
    }
  })

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault()

    if (submitting) return
    formError = ''

    if (!entry) return
    if (!formName.trim()) {
      formError = 'Name is required'
      return
    }

    submitting = true
    try {
      await onupdate(formCategoryId, { name: formName, icon: formIcon || null, color: formColor })
      toast.success('Category updated')
      open = false
    } catch (err) {
      formError = String(err)
    } finally {
      submitting = false
    }
  }
</script>

<Modal bind:open title="Edit category">
  <form onsubmit={handleSubmit} class="grid gap-y-4">
    <section class="flex flex-col gap-y-4 text-sm font-medium">
      <div class="grid gap-y-1">
        <label for="category-name">Name</label>
        <Input type="text" id="category-name" bind:value={formName} placeholder="My category" />
      </div>

      <div class="grid gap-y-1">
        <span id="category-icon-label">Icon</span>
        <div role="group" aria-labelledby="category-icon-label" class="flex flex-wrap gap-2">
          {#each defaultIcons as icon (icon)}
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
        {submitting ? 'Saving...' : 'Save changes'}
      </AlertDialog.Action>
    </section>
  </form>
</Modal>
