<script lang="ts">
	import type { PageData } from './$types';
	import Edit from 'carbon-icons-svelte/lib/Edit.svelte';
	import { writable } from 'svelte/store';
	import type { Writable } from 'svelte/store';
	import type { UserModel } from './+page.server';
	import { enhance } from '$app/forms';

	export let data: PageData;
	let index: Writable<UserModel | undefined> = writable(undefined);

	let buffer: UserModel = data.users[0];
	$: console.log(JSON.stringify(buffer));
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
						<a
							href="#modal"
							class="btn"
							on:click={() => {
								$index = item;
								buffer = item;
							}}><Edit /></a
						>
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
	<div class="modal" id="modal">
		<form
			method="post"
			class="modal-box"
			use:enhance={({ data }) => {
				data.set('registrationNumber', buffer.registrationNumber.toString());

				// return async (res) => {};
			}}
		>
			<h3 class="font-bold text-lg">Registration Number - {$index?.registrationNumber}</h3>
			<div class="pt-4">
				<label class="label" for="name">
					<span class="label-text">Name</span>
				</label>
				<input type="text" id="name" class="input input-bordered w-full" bind:value={buffer.name} />
			</div>
			<div class="py-2">
				<label class="label" for="title">
					<span class="label-text">Title</span>
				</label>
				<input
					type="text"
					id="title"
					class="input input-bordered w-full"
					bind:value={buffer.title}
				/>
			</div>
			<div class="py-2">
				<label class="label" for="phoneNumber">
					<span class="label-text">Phone Number</span>
				</label>
				<input
					type="text"
					id="phoneNumber"
					class="input input-bordered w-full"
					bind:value={buffer.phoneNumber}
				/>
			</div>
			<div class="py-2">
				<label class="label" for="email">
					<span class="label-text">Email</span>
				</label>
				<input
					type="text"
					id="email"
					class="input input-bordered w-full"
					bind:value={buffer.email}
				/>
			</div>
			<div class="py-2">
				<label class="label" for="designation">
					<span class="label-text">Designation</span>
				</label>
				<input
					type="text"
					id="designation"
					class="input input-bordered w-full"
					bind:value={buffer.designation}
				/>
			</div>
			<div class="py-2">
				<label class="label" for="department">
					<span class="label-text">Department</span>
				</label>
				<input
					type="text"
					id="department"
					class="input input-bordered w-full"
					bind:value={buffer.department}
				/>
			</div>
			<div class="py-2">
				<label class="label" for="year">
					<span class="label-text">Year</span>
				</label>
				<input type="text" id="year" class="input input-bordered w-full" bind:value={buffer.year} />
			</div>
			<div class="py-2">
				<label class="label" for="remarks">
					<span class="label-text">Remarks</span>
				</label>
				<input
					type="text"
					id="remarks"
					class="input input-bordered w-full"
					bind:value={buffer.remarks}
				/>
			</div>
			<div class="py-2">
				<label class="label" for="strikes">
					<span class="label-text">Strikes</span>
				</label>
				<input
					type="text"
					id="strikes"
					class="input input-bordered w-full"
					bind:value={buffer.strikes}
				/>
			</div>
			<div class="modal-action">
				<button type="submit" class="btn">Submit</button>
				<a href="#" class="btn">Close</a>
			</div>
		</form>
	</div>
</div>
