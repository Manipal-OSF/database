<script>
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
	import { Warning } from 'carbon-icons-svelte';
	import { writable } from 'svelte/store';
	import { fade, fly } from 'svelte/transition';

	let loading = writable(false);
	let error = writable(false);
</script>

<form
	method="post"
	class="h-full flex flex-col justify-center items-center gap-5"
	use:enhance={(_res) => {
		$loading = true;
		return async ({ result, update }) => {
			$loading = false;

			if (result.type !== 'success') {
				$error = true;

				setTimeout(() => {
					$error = false;
				}, 3000);
			} else {
				await goto('users');
			}

			update();
		};
	}}>
	<h1 class="text-5xl pb-10">OSF Database</h1>
	<input
		class="input input-bordered input-secondary"
		type="password"
		name="apikey"
		id="apikey"
		placeholder="API Key" />
	<button class="btn">
		{#if $loading}
			Loading
		{:else}
			Submit
		{/if}
	</button>
</form>
{#if $error}
	<div in:fly={{ y: 100 }} out:fade class="alert alert-error shadow-lg">
		<div>
			<Warning size={24} />
			<span>Error! Invalid credentials</span>
		</div>
	</div>
{/if}
