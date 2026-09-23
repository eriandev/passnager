import { resolve } from '$app/paths'
import { goto as svelteGoto } from '$app/navigation'
import type { RouteId } from '$app/types'

export function useNavigation() {
  const goto = (param: RouteId) => svelteGoto(resolve(param))

  return {
    goto,
  }
}
