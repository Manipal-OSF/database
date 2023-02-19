<script lang="ts">
	import type { PageData } from './$types';
	import Edit from 'carbon-icons-svelte/lib/Edit.svelte';
	import Add from 'carbon-icons-svelte/lib/Add.svelte';
	import { writable } from 'svelte/store';
	import type { Writable } from 'svelte/store';
	import type { UserModel } from './+page.server';
	import { enhance } from '$app/forms';
	import { fade, fly } from 'svelte/transition';
	import { CheckmarkOutline, Reset, Search, Warning } from 'carbon-icons-svelte';

	export let data: PageData;

	const defaultUser: UserModel = {
		registrationNumber: 0,
		name: '',
		title: '',
		phoneNumber: 0,
		email: '@.com',
		designation: '',
		department: '',
		year: 1,
		remarks: '',
		strikes: 0,
	};

	let filtered = writable(data.users);

	const filter = (search: string) => {
		if (search.trim() === '') {
			return;
		}

		// Get list of filtered users based on search regardless of column
		let result = data.users.filter((item) => {
			return Object.values(item).some((value) => {
				return String(value).toLowerCase().includes(search.toLowerCase());
			});
		});

		if (result.length !== 0) {
			$filtered = result;
		}
	};

	const reset = () => {
		$filtered = data.users;
		search = '';
	};

	let action: Writable<'update' | 'create'> = writable('update');
	let buffer: Writable<UserModel> = writable(defaultUser);
	let search = '';

	let alertState: Writable<'neutral' | 'error' | 'success'> = writable('neutral');
	let alertErrorMessage = '';

	const onSubmit = async () => {
		// Programmatically click the button as daisyUI does not expose JS controls to close modals.
		document.getElementById('close')?.click();
	};
</script>

<div class="w-screen h-screen">
	<div class="w-screen flex justify-between px-4 md:px-10 py-4">
		<!-- Filter UI -->
		<div class="form-control">
			<div class="input-group">
				<button on:click={() => reset()} class="btn  btn-square">
					<Reset size={24} />
				</button>
				<input
					bind:value={search}
					type="text"
					placeholder="Filter"
					class="input w-44 md:w-auto input-bordered" />
				<button on:click={() => filter(search)} class="btn btn-square">
					<Search size={24} />
				</button>
			</div>
		</div>
		<!-- Create a new user -->
		<a
			href="#modal"
			class="btn btn-square"
			on:click={() => {
				$action = 'create';
				$buffer = defaultUser;
			}}>
			<Add size={32} />
		</a>
	</div>
	<div class="px-4 md:px-10 py-4 overflow-x-scroll">
		<table class="table overflow-x-scroll hover w-full">
			<thead>
				<tr>
					<td class="border-blue-500 border-[1px]" />
					<th class="border-blue-500 border-[1px]">Registration Number</th>
					<th class="border-blue-500 border-[1px]">Name</th>
					<th class="border-blue-500 border-[1px]">Title</th>
					<th class="border-blue-500 border-[1px]">Phone Number</th>
					<th class="border-blue-500 border-[1px]">Email</th>
					<th class="border-blue-500 border-[1px]">Designation</th>
					<th class="border-blue-500 border-[1px]">Department</th>
					<th class="border-blue-500 border-[1px]">Year</th>
					<th class="border-blue-500 border-[1px]">Remarks</th>
					<th class="border-blue-500 border-[1px]">Strikes</th>
				</tr>
			</thead>
			<tbody>
				{#each $filtered as item, i}
					<tr>
						<td class="border-blue-300 border-[1px] ">
							<a
								href="#modal"
								class="btn"
								on:click={() => {
									$action = 'update';
									$buffer = item;
								}}>
								<Edit />
							</a>
						</td>
						<td class="border-blue-300 border-[1px]">{item.registrationNumber}</td>
						<td class="border-blue-300 border-[1px]">{item.name}</td>
						<td class="border-blue-300 border-[1px]">{item.title ?? 'NULL'}</td>
						<td class="border-blue-300 border-[1px]">{item.phoneNumber}</td>
						<td class="border-blue-300 border-[1px]">{item.email}</td>
						<td class="border-blue-300 border-[1px]">{item.designation ?? 'NULL'}</td>
						<td class="border-blue-300 border-[1px]">{item.department ?? 'NULL'}</td>
						<td class="border-blue-300 border-[1px]">{item.year}</td>
						<td class="border-blue-300 border-[1px]">{item.remarks ?? 'NULL'}</td>
						<td class="border-blue-300 border-[1px]">{item.strikes}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
	<div class="modal" id="modal">
		<form
			method="post"
			class="modal-box"
			use:enhance={({ data }) => {
				if ($action === 'update') {
					data.set('registrationNumber', $buffer.registrationNumber.toString());
					data.set('method', 'PATCH');
				} else {
					data.set('method', 'POST');
				}

				return async ({ result, update }) => {
					if (result.type === 'failure') {
						$alertState = 'error';
						alertErrorMessage = result.data?.message;
					} else {
						$alertState = 'success';
						await update({ reset: true });
					}

					// Remove the alert
					setTimeout(() => {
						$alertState = 'neutral';
					}, 3000);
				};
			}}>
			{#if $action === 'update'}
				<h3 class="font-bold text-lg">Update {$buffer.registrationNumber}</h3>
			{:else}
				<h3 class="font-bold text-lg">Create a new user</h3>
			{/if}
			<!-- Show registration number only when creating a user -->
			{#if $action === 'create'}
				<div class="pt-4">
					<label class="label" for="registrationNumber">
						<span class="label-text">Registration Number</span>
					</label>
					<input
						type="number"
						min="0"
						id="registrationNumber"
						name="registrationNumber"
						class="input input-bordered w-full"
						bind:value={$buffer.registrationNumber} />
				</div>
			{/if}
			<div class="pt-4">
				<label class="label" for="name">
					<span class="label-text">Name</span>
				</label>
				<input
					type="text"
					id="name"
					name="name"
					class="input input-bordered w-full"
					bind:value={$buffer.name} />
			</div>
			<div class="py-2">
				<label class="label" for="title">
					<span class="label-text">Title</span>
				</label>
				<input
					type="text"
					id="title"
					name="title"
					class="input input-bordered w-full"
					bind:value={$buffer.title} />
			</div>
			<div class="py-2">
				<label class="label" for="phoneNumber">
					<span class="label-text">Phone Number</span>
				</label>
				<input
					type="number"
					min="0"
					id="phoneNumber"
					name="phoneNumber"
					class="input input-bordered w-full"
					bind:value={$buffer.phoneNumber} />
			</div>
			<div class="py-2">
				<label class="label" for="email">
					<span class="label-text">Email</span>
				</label>
				<input
					type="email"
					id="email"
					name="email"
					class="input input-bordered w-full"
					bind:value={$buffer.email} />
			</div>
			<div class="py-2">
				<label class="label" for="designation">
					<span class="label-text">Designation</span>
				</label>
				<input
					type="text"
					id="designation"
					name="designation"
					class="input input-bordered w-full"
					bind:value={$buffer.designation} />
			</div>
			<div class="py-2">
				<label class="label" for="department">
					<span class="label-text">Department</span>
				</label>
				<input
					type="text"
					id="department"
					name="department"
					class="input input-bordered w-full"
					bind:value={$buffer.department} />
			</div>
			<div class="py-2">
				<label class="label" for="year">
					<span class="label-text">Year</span>
				</label>
				<input
					type="number"
					min="1"
					id="year"
					name="year"
					class="input input-bordered w-full"
					bind:value={$buffer.year} />
			</div>
			<div class="py-2">
				<label class="label" for="remarks">
					<span class="label-text">Remarks</span>
				</label>
				<input
					type="text"
					id="remarks"
					name="remarks"
					class="input input-bordered w-full"
					bind:value={$buffer.remarks} />
			</div>
			<div class="py-2">
				<label class="label" for="strikes">
					<span class="label-text">Strikes</span>
				</label>
				<input
					type="number"
					min="0"
					id="strikes"
					name="strikes"
					class="input input-bordered w-full"
					bind:value={$buffer.strikes} />
			</div>
			<div class="modal-action">
				<button type="submit" class="btn" on:click={() => onSubmit()}>Submit</button>
				<a href="#" id="close" class="btn">Close</a>
			</div>
		</form>
	</div>
</div>

{#if $alertState !== 'neutral'}
	<div
		in:fly={{ y: 100 }}
		out:fade
		class={`alert ${$alertState === 'error' ? 'alert-error' : 'alert-success'} shadow-lg`}>
		<div>
			{#if $alertState === 'error'}
				<Warning size={24} />
			{:else}
				<CheckmarkOutline size={24} />
			{/if}
			<span>
				{$alertState === 'error' ? alertErrorMessage : 'Success'}
			</span>
		</div>
	</div>
{/if}
