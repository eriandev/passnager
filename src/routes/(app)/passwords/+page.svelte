<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { useAuth } from '$lib/auth.svelte'
  import Plus from 'phosphor-svelte/lib/PlusIcon'
  import Input from '@/components/input.svelte'
  import Alert from '@/components/alert.svelte'
  import Button from '@/components/button.svelte'
  import { usePasswords } from '$lib/password.svelte'
  import PasswordCard from '@/components/password-card.svelte'
  import ModalAddPass from '@/components/modal-add-pass.svelte'
  import ModalEditPass from '@/components/modal-edit-pass.svelte'
  import type { EntryPasswordProps } from '$lib/types'

  const auth = useAuth()
  const passwords = usePasswords()

  let deleteTargetId = $state('')
  let showAddModal = $state(false)
  let showEditModal = $state(false)
  let showDeleteAlert = $state(false)
  let editingEntry = $state<EntryPasswordProps | null>(null)

  $effect(() => {
    if (auth.isUnlocked) {
      passwords.load().then((ok) => {
        if (!ok) toast.error('Failed to load passwords')
      })
    }
  })

  function openAdd() {
    showAddModal = true
  }

  function openEdit(entry: EntryPasswordProps) {
    editingEntry = entry
    showEditModal = true
  }

  function confirmDelete(id: string) {
    deleteTargetId = id
    showDeleteAlert = true
  }

  async function handleDelete() {
    console.log({ deleteTargetId })
    if (!deleteTargetId) return
    try {
      await passwords.remove(deleteTargetId)
      toast.success('Password deleted')
      showDeleteAlert = false
    } catch {
      toast.error('Failed to delete password')
    }
  }
</script>

<div class="mx-auto h-full grid grid-rows-[repeat(2,fit-content(100%))_minmax(0,1fr)] gap-y-6 max-w-4xl">
  <header class="flex items-center justify-between">
    <h1 class="text-3xl font-bold text-foreground">Passwords</h1>
  </header>

  <section>
    <Input placeholder="Search by username or URL" />
  </section>

  <section class="w-full h-full relative min-h-0 overflow-y-auto">
    {#if passwords.loading}
      <div class="grid h-full place-content-center text-foreground-alt">Loading...</div>
    {:else if passwords.list.length === 0}
      <div class="grid h-full place-content-center text-foreground-alt">No passwords saved yet. Add one!</div>
    {:else}
      <div class="flex flex-col gap-3">
        {#each passwords.list as entry (entry.id)}
          <PasswordCard {entry} onedit={openEdit} ondelete={confirmDelete} />
        {/each}
      </div>
    {/if}
  </section>

  <footer class="sticky w-full bottom-0 left-0 right-0 max-w-4xl mx-auto">
    <Button onclick={openAdd} class="flex gap-x-2">
      <Plus class="size-5" />
      Add password
    </Button>
  </footer>
</div>

<ModalAddPass bind:open={showAddModal} />
<ModalEditPass bind:open={showEditModal} entry={editingEntry} />
<Alert
  title="Delete password"
  bind:open={showDeleteAlert}
  description={['Are you sure you want to delete this password?', 'This action cannot be undone']}
  actions={[
    { label: 'Cancel' },
    {
      label: 'Delete',
      variant: 'danger',
      action: handleDelete,
    },
  ]}
/>
