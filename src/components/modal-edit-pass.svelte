<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { AlertDialog } from 'bits-ui'
  import Input from '@/components/input.svelte'
  import Modal from '@/components/modal.svelte'
  import Select from '@/components/select.svelte'
  import { usePasswords } from '$lib/password.svelte'
  import { useCategories } from '$lib/category.svelte'
  import type { EntryPasswordProps } from '@/lib/types'
  import type { ModalEditProps } from '@/components/types'

  const passwords = usePasswords()
  const categories = useCategories()

  let { open = $bindable(false), entry = null }: ModalEditProps<EntryPasswordProps> = $props()

  let formUrl = $state('')
  let formError = $state('')
  let formUsername = $state('')
  let formPassword = $state('')
  let submitting = $state(false)
  let formCategoryId = $state('')

  const categoryItems = $derived([
    { label: 'No category', value: '' },
    ...categories.list.map((cate) => ({ label: cate.icon + ' ' + cate.name, value: cate.id })),
  ])

  $effect(() => {
    if (open && entry) {
      formError = ''
      formPassword = ''
      formUrl = entry.url
      formUsername = entry.username
      formCategoryId = entry.categoryId || ''
    }
  })

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault()

    if (submitting) return
    formError = ''

    if (!entry) return
    if (!formUsername.trim()) {
      formError = 'Username is required'
      return
    }
    if (!formUrl.trim()) {
      formError = 'URL is required'
      return
    }

    submitting = true
    try {
      await passwords.update(entry.id, {
        url: formUrl,
        username: formUsername,
        password: formPassword || null,
        categoryId: formCategoryId || null,
      })
      toast.success('Password updated')
      open = false
    } catch (err) {
      formError = String(err)
    } finally {
      submitting = false
    }
  }
</script>

<Modal bind:open title="Edit password">
  <form onsubmit={handleSubmit} class="grid gap-y-4">
    <section class="flex flex-col gap-y-4 text-sm font-medium">
      <div class="grid gap-y-1">
        <label for="edit-pass-username">Username</label>
        <Input type="text" id="edit-pass-username" bind:value={formUsername} placeholder="user@example.com" />
      </div>

      <div class="grid gap-y-1">
        <label for="edit-pass-password">New password (leave empty to keep current)</label>
        <Input
          type="password"
          id="edit-pass-password"
          bind:value={formPassword}
          placeholder="Leave empty to keep current"
        />
      </div>

      <div class="grid gap-y-1">
        <label for="edit-pass-url">URL</label>
        <Input type="url" id="edit-pass-url" bind:value={formUrl} placeholder="https://example.com/login" />
      </div>

      <div class="grid gap-y-1">
        <label for="edit-pass-category">Category</label>
        <Select
          items={categoryItems}
          id="edit-pass-category"
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
      <AlertDialog.Cancel type="button" class="btn secondary" disabled={submitting}>
        Cancel
      </AlertDialog.Cancel>
      <AlertDialog.Action type="submit" class="btn primary" disabled={submitting}>
        {submitting ? 'Saving...' : 'Save changes'}
      </AlertDialog.Action>
    </section>
  </form>
</Modal>
