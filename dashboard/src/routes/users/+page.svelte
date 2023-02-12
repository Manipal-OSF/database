<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="overflow-x-auto">
	<table class="table w-full">
		<thead>
			<tr>
				<th>Index</th>
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
					<th>{i + 1}</th>
					<td>{item.registrationNumber}</td>
					<td>{item.name}</td>
					<td>{item.title}</td>
					<td>{item.phoneNumber}</td>
					<td>{item.email}</td>
					<td>{item.designation}</td>
					<td>{item.department}</td>
					<td>{item.year}</td>
					<td>{item.remarks}</td>
					<td>{item.strikes}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
