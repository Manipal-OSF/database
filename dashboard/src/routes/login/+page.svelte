<script>
	import { enhance } from '$app/forms';
</script>

<form method="post" use:enhance>
	<label>
		API Key
		<input type="password" name="apikey" />
	</label>
	<button>Submit</button>
</form>
