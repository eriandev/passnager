<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { AlertDialog } from 'bits-ui'
  import Input from '@/components/input.svelte'
  import Modal from '@/components/modal.svelte'
  import Select from '@/components/select.svelte'
  import { usePasswords } from '$lib/password.svelte'
  import { useCategories } from '$lib/category.svelte'
  import type { ModalAddPassProps } from '@/components/types'

  const passwords = usePasswords()
  const categories = useCategories()

  let { open = $bindable(false) }: ModalAddPassProps = $props()

  let formUrl = $state('')
  let formError = $state('')
  let formUsername = $state('')
  let formPassword = $state('')
  let formCategoryId = $state('')

  const categoryItems = $derived([
    { value: '', label: 'No category' },
    ...categories.list.map((c) => ({ value: c.id, label: c.name })),
  ])

  $effect(() => {
    if (open) {
      formUrl = ''
      formError = ''
      formUsername = ''
      formPassword = ''
      formCategoryId = ''
      categories.load()
    }
  })

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault()
    formError = ''

    if (!formUsername.trim()) {
      formError = 'Username is required'
      return
    }
    if (!formPassword) {
      formError = 'Password is required'
      return
    }
    if (!formUrl.trim()) {
      formError = 'URL is required'
      return
    }

    try {
      await passwords.add({
        url: formUrl,
        username: formUsername,
        password: formPassword,
        categoryId: formCategoryId || null,
      })
      toast.success('Password saved')
      open = false
    } catch (err) {
      formError = String(err)
    }
  }
</script>

<Modal bind:open title="New password">
  <form onsubmit={handleSubmit} class="grid gap-y-4">
    <section class="flex flex-col gap-y-4 text-sm font-medium">
      <div class="grid gap-y-1">
        <label for="add-pass-username">Username</label>
        <Input type="text" id="add-pass-username" bind:value={formUsername} placeholder="user@example.com" />
      </div>

      <div class="grid gap-y-1">
        <label for="add-pass-password">Password</label>
        <Input type="password" id="add-pass-password" bind:value={formPassword} placeholder="Your password" />
      </div>

      <div class="grid gap-y-1">
        <label for="add-pass-url">URL</label>
        <Input type="url" id="add-pass-url" bind:value={formUrl} placeholder="https://example.com/login" />
      </div>

      <div class="grid gap-y-1">
        <label for="add-pass-category">Category</label>
        <Select
          id="add-pass-category"
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
      <AlertDialog.Cancel type="button" class="btn secondary">Cancel</AlertDialog.Cancel>
      <AlertDialog.Action type="submit" class="btn primary">Add password</AlertDialog.Action>
    </section>
  </form>
</Modal>
