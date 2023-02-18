<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="h-screen flex flex-col items-center justify-center gap-5">
	<h1 class="text-5xl font-bold">Error</h1>
	<h2 class="text-3xl">{$page.error?.message}</h2>
	<a href="/" class="mt-5 btn">Go to Login</a>
</div>
