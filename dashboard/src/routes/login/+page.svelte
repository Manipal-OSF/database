<script>
	import { enhance } from '$app/forms';
	import { goto } from '$app/navigation';
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
				}, 1000);
			} else {
				await goto('users');
			}

			update();
		};
	}}>
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
			<svg
				xmlns="http://www.w3.org/2000/svg"
				class="stroke-current flex-shrink-0 h-6 w-6"
				fill="none"
				viewBox="0 0 24 24"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
			<span>Error! Task failed successfully.</span>
		</div>
	</div>
{/if}
