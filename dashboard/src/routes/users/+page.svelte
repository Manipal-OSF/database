<script lang="ts">
	import type { PageData } from './$types';
	import Edit from 'carbon-icons-svelte/lib/Edit.svelte';

	export let data: PageData;

	const edit = () => {};
</script>

<div class="overflow-x-auto w-full">
	<table class="table hover w-full">
		<thead>
			<tr>
				<th />
				<th>Registration Number</th>
				<th>Name</th>
				<th>Title</th>
				<th>Phone Number</th>
				<th>Email</th>
				<th>Designation</th>
				<th>Department</th>
				<th>Year</th>
				<th>Remarks</th>
				<th>Strikes</th>
			</tr>
		</thead>
		<tbody>
			{#each data.users as item, i}
				<tr>
					<th>
						<button on:click={() => edit()}><Edit /></button>
					</th>
					<th>{item.registrationNumber}</th>
					<td>{item.name}</td>
					<td>{item.title ?? 'NULL'}</td>
					<td>{item.phoneNumber}</td>
					<td>{item.email}</td>
					<td>{item.designation ?? 'NULL'}</td>
					<td>{item.department ?? 'NULL'}</td>
					<td>{item.year}</td>
					<td>{item.remarks ?? 'NULL'}</td>
					<td>{item.strikes}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
