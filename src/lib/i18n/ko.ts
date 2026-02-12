const ko = {
	// Common
	'common.cancel': '취소',
	'common.save': '저장',
	'common.delete': '삭제',
	'common.search': '검색...',
	'common.edit': '수정',

	// Malt types (display labels)
	'type.결정': '결정',
	'type.문제': '문제',
	'type.인사이트': '인사이트',
	'type.질문': '질문',
	'type.결정.desc': '확정된 의사결정',
	'type.문제.desc': '해결이 필요한 이슈',
	'type.인사이트.desc': '발견한 통찰',
	'type.질문.desc': '논의가 필요한 질문',

	// Form
	'form.summary': '한 줄 요약 *',
	'form.context': '맥락 (선택)',
	'form.memo': '메모 (선택)',
	'form.submit': '몰팅',

	// Nav
	'nav.malting': '몰팅',
	'nav.maltHouse': '몰트 하우스',
	'nav.still': '스틸',
	'nav.drawback': '드로우백',
	'nav.help': '도움말',
	'nav.settings': '설정',

	// Malting page
	'malting.title': '몰팅',
	'malting.subtitle': '생각을 몰트로 만드세요.',

	// Malt House page
	'maltHouse.title': '몰트 하우스',
	'maltHouse.subtitle': '내 몰트를 보관하는 곳.',

	// Still page
	'still.title': '스틸',
	'still.subtitle': '스틸에 올린 몰트를 증류하세요.',
	'still.queueAll': '모두 대기열 추가',

	// Drawback page
	'drawback.title': '드로우백',
	'drawback.subtitle': '증류 대기열에서 몰트를 빼낼 수 있습니다.',

	// MaltCard
	'card.putOnStill': '스틸에 올리기',
	'card.takeOffStill': '스틸에서 내리기',
	'card.queue': '대기열 추가',
	'card.drawBack': '드로우백',

	// MaltList
	'list.listView': '리스트 뷰',
	'list.cardView': '카드 뷰',
	'list.emptySearch': '검색 결과가 없습니다.',
	'list.emptyStill': '스틸에 올린 몰트가 없습니다.',
	'list.emptyDrawback': '증류 대기열이 비어있습니다.',
	'list.emptyMalts': '아직 몰트가 없습니다.',
	'list.ctaStill': '몰트 하우스에서 올리기',
	'list.ctaDrawback': '스틸에서 대기열 추가하기',
	'list.ctaMalts': '첫 몰트 만들기',

	// Distill Timer
	'timer.label': '다음 증류까지',
	'timer.hours': '시간',
	'timer.minutes': '분',

	// Login
	'login.title': '증류소 출입 인증',
	'login.email': '이메일 주소',
	'login.sendOtp': '출입 인증 요청',
	'login.otpSent': 'Slack DM으로 전송된 출입 코드를 입력하세요.',
	'login.verify': '입장하기',
	'login.tryOther': '다른 이메일로 시도',

	// Settings
	'settings.title': '설정',
	'settings.account': '계정',
	'settings.authenticated': '증류소 출입 인증됨',
	'settings.logout': '퇴장',
	'settings.layout': '레이아웃',
	'settings.sidebarPosition': '사이드바 위치',
	'settings.sidebarDesc': '사이드바를 원하는 위치에 배치합니다.',
	'settings.pos.top': '위',
	'settings.pos.left': '왼쪽',
	'settings.pos.right': '오른쪽',
	'settings.pos.bottom': '아래',
	'settings.language': '언어',
	'settings.languageDesc': '앱 표시 언어를 선택합니다.',

	// Help
	'help.title': '도움말',
	'help.subtitle': 'Distillery 사용 가이드',
	'help.concept': '컨셉',
	'help.conceptDesc': 'Distillery는 위스키 증류소의 과정을 모티브로 합니다. 생각을 몰트로 만들고, 스틸에 올려 선별한 뒤, 증류 대기열에 넣으면 서버가 자동으로 증류하여 캐스크에서 숙성시킵니다.',
	'help.step1.title': '몰팅 (Malting)',
	'help.step1.desc': '생각을 <b>몰트</b>로 만드는 과정입니다. 결정, 문제, 인사이트, 질문 중 유형을 선택하고, 한 줄 요약을 작성하세요. 맥락과 메모는 선택사항입니다.',
	'help.step2.title': '몰트 하우스 (Malt House)',
	'help.step2.desc': '내 몰트를 보관하는 곳입니다. 몰트를 수정하거나 삭제할 수 있습니다. <b>스틸에 올리기</b> 아이콘을 눌러 증류할 몰트를 선별하세요.',
	'help.step3.title': '스틸 (Still)',
	'help.step3.desc': '스틸에 올린 몰트를 관리합니다. <b>대기열 추가</b> 버튼으로 증류 대기열에 넣을 수 있습니다. 대기열에 넣기 전이라면 <b>스틸에서 내려</b> 몰트 하우스로 되돌릴 수 있습니다.',
	'help.step4.title': '드로우백 (Draw Back)',
	'help.step4.desc1': '증류 대기열에 있는 몰트를 빼냅니다. 서버에서 삭제되고, 스틸로 되돌아갑니다.',
	'help.step4.desc2': '서버는 매일 <b>정오(12:00)</b>와 <b>자정(00:00)</b>에 대기열의 몰트를 증류하여 <b>캐스크(지식 그래프)</b>에 넣습니다. 이미 캐스크에 들어간 몰트는 빼낼 수 없습니다.',
	'help.glossary': '용어 사전',
	'help.g.malting': '몰팅 (Malting)',
	'help.g.malting.d': '생각을 몰트로 만드는 과정.',
	'help.g.malt': '몰트 (Malt)',
	'help.g.malt.d': '개별 기록. 증류의 원재료.',
	'help.g.maltHouse': '몰트 하우스',
	'help.g.maltHouse.d': '내 몰트를 보관하는 곳.',
	'help.g.putOnStill': '스틸에 올리기',
	'help.g.putOnStill.d': '증류할 몰트를 선별하여 스틸에 올리는 행위.',
	'help.g.takeOffStill': '스틸에서 내리기',
	'help.g.takeOffStill.d': '몰트를 내려 몰트 하우스로 되돌리는 행위.',
	'help.g.still': '스틸 (Still)',
	'help.g.still.d': '선별된 몰트를 관리하는 곳.',
	'help.g.queue': '증류 대기열',
	'help.g.queue.d': '서버로 전송된 몰트. 증류를 기다리는 상태.',
	'help.g.distill': '증류 (Distill)',
	'help.g.distill.d': '서버가 정오/자정에 자동으로 처리하는 과정.',
	'help.g.cask': '캐스크 (Cask)',
	'help.g.cask.d': '증류 완료 후 숙성되는 곳. 지식 그래프.',
	'help.g.drawBack': '드로우백',
	'help.g.drawBack.d': '대기열에서 몰트를 빼내기. 캐스크에 들어간 건 불가.',
	'help.maltTypes': '몰트 유형',
	'help.flow': '흐름',
	'help.flow.malting': '몰팅',
	'help.flow.maltHouse': '몰트 하우스',
	'help.flow.still': '스틸',
	'help.flow.queue': '대기열',
	'help.flow.distill': '증류',
	'help.flow.cask': '캐스크'
} as const;

export default ko;
export type MessageKey = keyof typeof ko;
export type Messages = Record<MessageKey, string>;
