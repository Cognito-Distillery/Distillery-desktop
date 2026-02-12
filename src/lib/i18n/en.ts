import type { Messages } from './ko';

const en: Messages = {
	// Common
	'common.cancel': 'Cancel',
	'common.save': 'Save',
	'common.delete': 'Delete',
	'common.search': 'Search...',
	'common.edit': 'Edit',

	// Malt types (display labels)
	'type.결정': 'Decision',
	'type.문제': 'Problem',
	'type.인사이트': 'Insight',
	'type.질문': 'Question',
	'type.결정.desc': 'Confirmed decisions',
	'type.문제.desc': 'Issues that need resolution',
	'type.인사이트.desc': 'Discovered insights',
	'type.질문.desc': 'Questions for discussion',

	// Form
	'form.summary': 'One-line summary *',
	'form.context': 'Context (optional)',
	'form.memo': 'Memo (optional)',
	'form.submit': 'Malt',

	// Nav
	'nav.malting': 'Malting',
	'nav.maltHouse': 'Malt House',
	'nav.still': 'Still',
	'nav.drawback': 'Draw Back',
	'nav.help': 'Help',
	'nav.settings': 'Settings',

	// Malting page
	'malting.title': 'Malting',
	'malting.subtitle': 'Turn your thoughts into malts.',

	// Malt House page
	'maltHouse.title': 'Malt House',
	'maltHouse.subtitle': 'Where your malts are stored.',

	// Still page
	'still.title': 'Still',
	'still.subtitle': 'Distill the malts on the still.',
	'still.queueAll': 'Queue All',

	// Drawback page
	'drawback.title': 'Draw Back',
	'drawback.subtitle': 'Remove malts from the distill queue.',

	// MaltCard
	'card.putOnStill': 'Put on Still',
	'card.takeOffStill': 'Take off Still',
	'card.queue': 'Queue',
	'card.drawBack': 'Draw Back',

	// MaltList
	'list.listView': 'List view',
	'list.cardView': 'Card view',
	'list.emptySearch': 'No results found.',
	'list.emptyStill': 'No malts on the still.',
	'list.emptyDrawback': 'The distill queue is empty.',
	'list.emptyMalts': 'No malts yet.',
	'list.ctaStill': 'Put from Malt House',
	'list.ctaDrawback': 'Queue from Still',
	'list.ctaMalts': 'Create your first malt',

	// Distill Timer
	'timer.label': 'Next distill in',
	'timer.hours': 'h',
	'timer.minutes': 'm',

	// Login
	'login.title': 'Distillery Access',
	'login.email': 'Email address',
	'login.sendOtp': 'Request Access Code',
	'login.otpSent': 'Enter the access code sent via Slack DM.',
	'login.verify': 'Enter',
	'login.tryOther': 'Try another email',

	// Settings
	'settings.title': 'Settings',
	'settings.account': 'Account',
	'settings.authenticated': 'Distillery access verified',
	'settings.logout': 'Leave',
	'settings.layout': 'Layout',
	'settings.sidebarPosition': 'Sidebar position',
	'settings.sidebarDesc': 'Place the sidebar where you want.',
	'settings.pos.top': 'Top',
	'settings.pos.left': 'Left',
	'settings.pos.right': 'Right',
	'settings.pos.bottom': 'Bottom',
	'settings.language': 'Language',
	'settings.languageDesc': 'Choose the display language.',

	// Help
	'help.title': 'Help',
	'help.subtitle': 'Distillery User Guide',
	'help.concept': 'Concept',
	'help.conceptDesc': 'Distillery is inspired by the whiskey distillation process. Create malts from your thoughts, put them on the still to select, then queue for distillation. The server automatically distills and ages them in casks.',
	'help.step1.title': 'Malting',
	'help.step1.desc': 'The process of turning thoughts into <b>malts</b>. Choose a type — Decision, Problem, Insight, or Question — and write a one-line summary. Context and memo are optional.',
	'help.step2.title': 'Malt House',
	'help.step2.desc': 'Where your malts are stored. You can edit or delete malts. Use the <b>Put on Still</b> icon to select malts for distillation.',
	'help.step3.title': 'Still',
	'help.step3.desc': 'Manage malts on the still. Use the <b>Queue</b> button to add to the distill queue. Before queuing, you can <b>take them off</b> to return to the Malt House.',
	'help.step4.title': 'Draw Back',
	'help.step4.desc1': 'Remove malts from the distill queue. They are deleted from the server and returned to the still.',
	'help.step4.desc2': 'The server distills queued malts daily at <b>noon (12:00)</b> and <b>midnight (00:00)</b>, placing them in the <b>Cask (knowledge graph)</b>. Malts already in the cask cannot be drawn back.',
	'help.glossary': 'Glossary',
	'help.g.malting': 'Malting',
	'help.g.malting.d': 'The process of turning thoughts into malts.',
	'help.g.malt': 'Malt',
	'help.g.malt.d': 'An individual record. Raw material for distillation.',
	'help.g.maltHouse': 'Malt House',
	'help.g.maltHouse.d': 'Where your malts are stored.',
	'help.g.putOnStill': 'Put on Still',
	'help.g.putOnStill.d': 'Selecting malts for distillation by placing them on the still.',
	'help.g.takeOffStill': 'Take off Still',
	'help.g.takeOffStill.d': 'Removing malts and returning them to the Malt House.',
	'help.g.still': 'Still',
	'help.g.still.d': 'Where selected malts are managed.',
	'help.g.queue': 'Distill Queue',
	'help.g.queue.d': 'Malts sent to the server, awaiting distillation.',
	'help.g.distill': 'Distill',
	'help.g.distill.d': 'Automatic processing by the server at noon/midnight.',
	'help.g.cask': 'Cask',
	'help.g.cask.d': 'Where distilled malts age. The knowledge graph.',
	'help.g.drawBack': 'Draw Back',
	'help.g.drawBack.d': 'Removing malts from the queue. Not possible after casking.',
	'help.maltTypes': 'Malt Types',
	'help.flow': 'Flow',
	'help.flow.malting': 'Malting',
	'help.flow.maltHouse': 'Malt House',
	'help.flow.still': 'Still',
	'help.flow.queue': 'Queue',
	'help.flow.distill': 'Distill',
	'help.flow.cask': 'Cask',

	// Floating
	'floating.title': 'Quick Malt'
};

export default en;
