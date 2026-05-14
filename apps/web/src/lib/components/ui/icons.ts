import CaretLeftIcon from 'phosphor-svelte/lib/CaretLeftIcon';
import CaretRightIcon from 'phosphor-svelte/lib/CaretRightIcon';
import CaretUpIcon from 'phosphor-svelte/lib/CaretUpIcon';
import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon';
import ShuffleIcon from 'phosphor-svelte/lib/ShuffleIcon';
import RepeatIcon from 'phosphor-svelte/lib/RepeatIcon';
import RepeatOnceIcon from 'phosphor-svelte/lib/RepeatOnceIcon';
import XIcon from 'phosphor-svelte/lib/XIcon';
import CheckIcon from 'phosphor-svelte/lib/CheckIcon';
import MinusIcon from 'phosphor-svelte/lib/MinusIcon';
import PlusIcon from 'phosphor-svelte/lib/PlusIcon';
import MagnifyingGlassIcon from 'phosphor-svelte/lib/MagnifyingGlassIcon';
import DotsThreeIcon from 'phosphor-svelte/lib/DotsThreeIcon';
import DotsSixVerticalIcon from 'phosphor-svelte/lib/DotsSixVerticalIcon';
import SidebarIcon from 'phosphor-svelte/lib/SidebarIcon';
import PencilSimpleIcon from 'phosphor-svelte/lib/PencilSimpleIcon';
import PlayIcon from 'phosphor-svelte/lib/PlayIcon';
import PauseIcon from 'phosphor-svelte/lib/PauseIcon';
import SkipBackIcon from 'phosphor-svelte/lib/SkipBackIcon';
import SkipForwardIcon from 'phosphor-svelte/lib/SkipForwardIcon';
import SpeakerHighIcon from 'phosphor-svelte/lib/SpeakerHighIcon';
import SpeakerSlashIcon from 'phosphor-svelte/lib/SpeakerSlashIcon';
import MusicNoteIcon from 'phosphor-svelte/lib/MusicNoteIcon';
import PlaylistIcon from 'phosphor-svelte/lib/PlaylistIcon';
import MicrophoneStageIcon from 'phosphor-svelte/lib/MicrophoneStageIcon';
import TagIcon from 'phosphor-svelte/lib/TagIcon';
import SquaresFourIcon from 'phosphor-svelte/lib/SquaresFourIcon';
import SparkleIcon from 'phosphor-svelte/lib/SparkleIcon';
import ListPlusIcon from 'phosphor-svelte/lib/ListPlusIcon';
import FolderPlusIcon from 'phosphor-svelte/lib/FolderPlusIcon';

// Lucide fallbacks (no clean phosphor equivalent)
import ListStartIcon from '@lucide/svelte/icons/list-start';
import ListEndIcon from '@lucide/svelte/icons/list-end';

export const Icons = {
	// Navigation / carets
	GoLeft: CaretLeftIcon,
	GoRight: CaretRightIcon,
	ChevronUp: CaretUpIcon,
	ChevronDown: CaretDownIcon,
	SortAsc: CaretUpIcon,
	SortDesc: CaretDownIcon,

	// Common UI
	Close: XIcon,
	Check: CheckIcon,
	Minus: MinusIcon,
	Add: PlusIcon,
	Edit: PencilSimpleIcon,
	Search: MagnifyingGlassIcon,
	More: DotsThreeIcon,
	Grip: DotsSixVerticalIcon,
	Sidebar: SidebarIcon,

	// Player controls
	Play: PlayIcon,
	Pause: PauseIcon,
	SkipBack: SkipBackIcon,
	SkipForward: SkipForwardIcon,
	Shuffle: ShuffleIcon,
	Repeat: RepeatIcon,
	RepeatOne: RepeatOnceIcon,
	Volume: SpeakerHighIcon,
	VolumeMute: SpeakerSlashIcon,

	// Entities
	Music: MusicNoteIcon,
	Playlist: PlaylistIcon,
	Artist: MicrophoneStageIcon,
	Tag: TagIcon,
	Collection: SquaresFourIcon,
	Sparkle: SparkleIcon,

	// List actions
	AddToList: ListPlusIcon,
	AddFolder: FolderPlusIcon,
	PlayNext: ListStartIcon,
	AddToQueue: ListEndIcon,
};
