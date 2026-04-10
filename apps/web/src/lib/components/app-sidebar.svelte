<script lang="ts">
	import { goto } from '$app/navigation';
	import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import type { ComponentProps } from 'svelte';

	let {
		user,
		ref = $bindable(null),
		...restProps
	}: ComponentProps<typeof Sidebar.Root> & { user: CurrentUserResponse } = $props();

	async function signOut() {
		await fetch('/api/auth/sign-out', { method: 'POST' });
		goto('/login');
	}
</script>

<Sidebar.Root {...restProps} bind:ref>
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton size="lg">
					{#snippet child({ props })}
						<DropdownMenu.Root {...props}>
							<DropdownMenu.Trigger>
								{#snippet child({ props })}
									<Sidebar.MenuButton {...props} size="lg">{user.name}</Sidebar.MenuButton>
								{/snippet}
							</DropdownMenu.Trigger>
							<DropdownMenu.Content class="w-56" align="start">
								<DropdownMenu.Group>
									<DropdownMenu.Sub>
										<DropdownMenu.SubTrigger>Invite users</DropdownMenu.SubTrigger>
										<DropdownMenu.SubContent>
											<DropdownMenu.Item>Email</DropdownMenu.Item>
											<DropdownMenu.Item>Message</DropdownMenu.Item>
											<DropdownMenu.Separator />
											<DropdownMenu.Item>More...</DropdownMenu.Item>
										</DropdownMenu.SubContent>
									</DropdownMenu.Sub>
								</DropdownMenu.Group>
								<DropdownMenu.Separator />
								<DropdownMenu.Item onclick={signOut}>Sign out</DropdownMenu.Item>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton>
						{#snippet child({ props })}
							<a href="/" {...props}>Home</a>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			</Sidebar.Menu>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Rail />
</Sidebar.Root>
