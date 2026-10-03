<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { AlertDialog } from 'bits-ui'
  import Input from '@/components/input.svelte'
  import Modal from '@/components/modal.svelte'
  import Select from '@/components/select.svelte'
  import Textarea from '@/components/textarea.svelte'
  import { categoryLabel } from '$lib/category.svelte'
  import ColorPicker from '@/components/color-picker.svelte'
  import { NOTE_CONTENT_MAX_CHARS, NOTE_DEFAULT_COLOR, NOTE_TITLE_MAX_CHARS } from '$lib/consts'
  import type { ModalEditNoteProps } from '@/components/types'

  let { open = $bindable(false), entry = null, onupdate, categories }: ModalEditNoteProps = $props()

  let formTitle = $state('')
  let formError = $state('')
  let formContent = $state('')
  let submitting = $state(false)
  let formColor = $state(NOTE_DEFAULT_COLOR)
  let formCategoryId = $state<string | null>(null)

  const contentLength = $derived(Array.from(formContent).length)
  const contentTooLong = $derived(contentLength > NOTE_CONTENT_MAX_CHARS)

  const categoryItems = $derived([
    { label: 'No category', value: '' },
    ...categories.map((cate) => ({ label: categoryLabel(cate), value: cate.id })),
  ])

  $effect(() => {
    if (open && entry) {
      formError = ''
      formContent = ''
      formTitle = entry.title
      formCategoryId = entry.categoryId
      formColor = entry.color ?? NOTE_DEFAULT_COLOR
    }
  })

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault()

    if (submitting) return
    formError = ''

    if (!entry) return
    if (!formTitle.trim()) {
      formError = 'Title is required'
      return
    }
    if (Array.from(formTitle).length > NOTE_TITLE_MAX_CHARS) {
      formError = `Title must be ${NOTE_TITLE_MAX_CHARS} characters or fewer`
      return
    }
    if (contentTooLong) {
      formError = `Content must be ${NOTE_CONTENT_MAX_CHARS} characters or fewer`
      return
    }

    submitting = true
    try {
      await onupdate(entry.id, {
        title: formTitle.trim(),
        content: formContent || null,
        color: formColor,
        categoryId: formCategoryId,
      })
      toast.success('Note updated')
      open = false
    } catch (err) {
      formError = String(err)
    } finally {
      submitting = false
    }
  }
</script>

<Modal bind:open title="Edit note">
  <form onsubmit={handleSubmit} class="grid gap-y-4">
    <section class="flex flex-col gap-y-4 text-sm font-medium">
      <div class="grid gap-y-1">
        <label for="edit-note-title">Title</label>
        <Input type="text" id="edit-note-title" bind:value={formTitle} placeholder="Router admin details" />
      </div>

      <div class="grid gap-y-1">
        <div class="flex items-center justify-between">
          <label for="edit-note-content">New content (leave empty to keep current)</label>
          <span class={['text-xs font-normal', contentTooLong ? 'text-destructive' : 'text-foreground-alt']}>
            {contentLength}/{NOTE_CONTENT_MAX_CHARS}
          </span>
        </div>
        <Textarea
          rows={6}
          id="edit-note-content"
          bind:value={formContent}
          aria-invalid={contentTooLong}
          placeholder="Leave empty to keep current"
        />
      </div>

      <div class="grid gap-y-1">
        <label for="edit-note-color">Color</label>
        <ColorPicker id="edit-note-color" bind:value={formColor} />
      </div>

      <div class="grid gap-y-1">
        <label for="edit-note-category">Category</label>
        <Select
          items={categoryItems}
          id="edit-note-category"
          placeholder="No category"
          onValueChange={(v) => (formCategoryId = v)}
        />
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
