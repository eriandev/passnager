<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { AlertDialog } from 'bits-ui'
  import { useNotes } from '$lib/note.svelte'
  import Input from '@/components/input.svelte'
  import Modal from '@/components/modal.svelte'
  import Select from '@/components/select.svelte'
  import Textarea from '@/components/textarea.svelte'
  import { useCategories } from '$lib/category.svelte'
  import { NOTE_CONTENT_MAX, NOTE_DEFAULT_COLOR } from '$lib/consts'
  import type { ModalAddProps } from '@/components/types'

  const notes = useNotes()
  const categories = useCategories()

  let { open = $bindable(false) }: ModalAddProps = $props()

  let formTitle = $state('')
  let formError = $state('')
  let formContent = $state('')
  let submitting = $state(false)
  let formCategoryId = $state('')
  let formColor = $state(NOTE_DEFAULT_COLOR)

  const contentLength = $derived([...formContent].length)
  const contentTooLong = $derived(contentLength > NOTE_CONTENT_MAX)

  const categoryItems = $derived([
    { label: 'No category', value: '' },
    ...categories.list.map((cate) => ({ label: cate.icon + ' ' + cate.name, value: cate.id })),
  ])

  $effect(() => {
    if (open) {
      formTitle = ''
      formError = ''
      formContent = ''
      formCategoryId = ''
      formColor = NOTE_DEFAULT_COLOR
    }
  })

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault()

    if (submitting) return
    formError = ''

    if (!formTitle.trim()) {
      formError = 'Title is required'
      return
    }
    if (!formContent.trim()) {
      formError = 'Content is required'
      return
    }
    if (contentTooLong) {
      formError = `Content must be ${NOTE_CONTENT_MAX} characters or fewer`
      return
    }

    submitting = true
    try {
      await notes.add({
        color: formColor,
        content: formContent,
        title: formTitle.trim(),
        categoryId: formCategoryId || null,
      })
      toast.success('Note saved')
      open = false
    } catch (err) {
      formError = String(err)
    } finally {
      submitting = false
    }
  }
</script>

<Modal bind:open title="New note">
  <form onsubmit={handleSubmit} class="grid gap-y-4">
    <section class="flex flex-col gap-y-4 text-sm font-medium">
      <div class="grid gap-y-1">
        <label for="add-note-title">Title</label>
        <Input type="text" id="add-note-title" bind:value={formTitle} placeholder="Note title" />
      </div>

      <div class="grid gap-y-1">
        <div class="flex items-center justify-between">
          <label for="add-note-content">Content</label>
          <span class={['text-xs font-normal', contentTooLong ? 'text-destructive' : 'text-foreground-alt']}>
            {contentLength}/{NOTE_CONTENT_MAX}
          </span>
        </div>
        <Textarea
          rows={6}
          id="add-note-content"
          bind:value={formContent}
          aria-invalid={contentTooLong}
          placeholder="Anything worth keeping"
        />
      </div>

      <div class="grid gap-y-1">
        <label for="add-note-color">Color</label>
        <input
          type="color"
          id="add-note-color"
          bind:value={formColor}
          class="h-10 w-20 rounded-lg border border-border-input bg-background"
        />
      </div>

      <div class="grid gap-y-1">
        <label for="add-note-category">Category</label>
        <Select
          id="add-note-category"
          items={categoryItems}
          placeholder="No category"
          bind:value={formCategoryId}
          onValueChange={(v) => (formCategoryId = v ?? '')}
        />
      </div>
    </section>

    <span class={['min-h-4 text-xs font-bold text-destructive', { invisible: !formError }]}>
      {formError}
    </span>

    <section class="flex justify-end gap-x-3">
      <AlertDialog.Cancel type="button" class="btn secondary" disabled={submitting}>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action type="submit" class="btn primary" disabled={submitting}>
        {submitting ? 'Adding...' : 'Add note'}
      </AlertDialog.Action>
    </section>
  </form>
</Modal>
