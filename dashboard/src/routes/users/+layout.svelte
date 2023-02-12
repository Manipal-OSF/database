<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

{#if data.token}
	<slot />
{:else}
	<h1>Error</h1>
{/if}
