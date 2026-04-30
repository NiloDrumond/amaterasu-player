import Root from './context-menu.svelte';
import Trigger from './context-menu-trigger.svelte';
import Content from './context-menu-content.svelte';
import Portal from './context-menu-portal.svelte';
import {
	Group,
	GroupHeading,
	Item,
	Label,
	Separator,
	Shortcut,
} from '$lib/components/ui/dropdown-menu/index.js';

export {
	Root,
	Root as ContextMenu,
	Trigger,
	Trigger as ContextMenuTrigger,
	Content,
	Content as ContextMenuContent,
	Portal,
	Portal as ContextMenuPortal,
	Group,
	Group as ContextMenuGroup,
	GroupHeading,
	GroupHeading as ContextMenuGroupHeading,
	Item,
	Item as ContextMenuItem,
	Label,
	Label as ContextMenuLabel,
	Separator,
	Separator as ContextMenuSeparator,
	Shortcut,
	Shortcut as ContextMenuShortcut,
};
