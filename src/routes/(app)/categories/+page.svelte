<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { useAuth } from '$lib/auth.svelte'
  import Plus from 'phosphor-svelte/lib/PlusIcon'
  import Alert from '@/components/alert.svelte'
  import Button from '@/components/button.svelte'
  import CategoryCard from '@/components/card-category.svelte'
  import ModalAddCate from '@/components/modal-add-cate.svelte'
  import ModalEditCate from '@/components/modal-edit-cate.svelte'
  import { useCategories } from '$lib/category.svelte'
  import type { EntryCategoryProps } from '$lib/types'

  const auth = useAuth()
  const categories = useCategories()

  let deleting = $state(false)
  let deleteTargetId = $state('')
  let showAddModal = $state(false)
  let showEditModal = $state(false)
  let showDeleteAlert = $state(false)
  let editingEntry = $state<EntryCategoryProps | null>(null)

  $effect(() => {
    if (auth.isUnlocked) {
      categories.load().then((ok) => {
        if (!ok) toast.error('Failed to load categories')
      })
    }
  })

  function openAdd() {
    showAddModal = true
  }

  function openEdit(entry: EntryCategoryProps) {
    editingEntry = entry
    showEditModal = true
  }

  function confirmDelete(id: string) {
    deleteTargetId = id
    showDeleteAlert = true
  }

  async function handleDelete() {
    if (!deleteTargetId || deleting) return

    // The id is read into a local before the await. Reading it afterwards would
    // pick up whatever the dialog was pointed at in the meantime, so the row that
    // got deleted and the row that was confirmed could be two different rows.
    const id = deleteTargetId
    deleting = true

    try {
      await categories.remove(id)
      // Only once the row is gone: while it is still there, a second confirm would
      // be a second request against the same row.
      deleteTargetId = ''
      toast.success('Category deleted')
      showDeleteAlert = false
    } catch {
      toast.error('Failed to delete category')
    } finally {
      deleting = false
    }
  }
</script>

<div class="mx-auto h-full grid grid-rows-[fit-content(100%)_minmax(0,1fr)] gap-y-6 max-w-4xl">
  <header class="flex items-center justify-between">
    <h1 class="text-3xl font-bold text-foreground">Categories</h1>
  </header>

  <section class="w-full h-full relative min-h-0 overflow-y-auto">
    {#if categories.loading}
      <div class="grid h-full place-content-center text-foreground-alt">Loading...</div>
    {:else if categories.list.length === 0}
      <div class="grid text-center h-full place-content-center text-foreground-alt">
        <p>No categories yet.</p>
        <p>Create one to organize your passwords!</p>
      </div>
    {:else}
      <div class="flex flex-col gap-3">
        {#each categories.list as entry (entry.id)}
          <CategoryCard {entry} onedit={openEdit} ondelete={confirmDelete} />
        {/each}
      </div>
    {/if}
  </section>

  <footer class="sticky w-full bottom-0 left-0 right-0 max-w-4xl mx-auto">
    <Button onclick={openAdd} class="flex gap-x-2">
      <Plus class="size-5" />
      Add category
    </Button>
  </footer>
</div>

<ModalAddCate bind:open={showAddModal} onadd={categories.add} />
<ModalEditCate bind:open={showEditModal} entry={editingEntry} onupdate={categories.update} />
<Alert
  title="Delete category"
  bind:open={showDeleteAlert}
  description={['Are you sure?', 'Associated passwords will not be deleted, they will just lose the category']}
  actions={[
    { label: 'Cancel' },
    {
      label: 'Delete',
      variant: 'danger',
      disabled: deleting,
      action: handleDelete,
    },
  ]}
/>
