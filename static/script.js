const root = document.documentElement;
const header = document.querySelector('[data-header]');
const menuButton = document.querySelector('[data-menu-button]');
const mobileMenu = document.querySelector('[data-mobile-menu]');
const themeToggle = document.querySelector('[data-theme-toggle]');
const themeLabel = document.querySelector('[data-theme-label]');
const langPickers = Array.from(document.querySelectorAll('[data-lang-picker]'));
const mobileLangRows = Array.from(document.querySelectorAll('[data-mobile-lang-row]'));
const revealItems = Array.from(document.querySelectorAll('[data-reveal]'));
const heroBg = document.querySelector('[data-hero-bg]');
const motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
const themeStorageKey = 'zkrypto-theme';
const languageStorageKey = 'zkrypto-language';
const isProductPage = document.body.classList.contains('product-page');

const LANGUAGES = [
  { code: 'ko', label: '한국어', short: 'KO' },
  { code: 'en', label: 'English', short: 'EN' },
];

function getSystemTheme() {
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function getStoredTheme() {
  const stored = window.localStorage.getItem(themeStorageKey);
  return stored === 'dark' || stored === 'light' ? stored : null;
}

function getStoredLanguage() {
  const stored = window.localStorage.getItem(languageStorageKey);
  return LANGUAGES.some((l) => l.code === stored) ? stored : null;
}

function applyTheme(theme, persist = false) {
  root.dataset.theme = theme;
  if (themeToggle) themeToggle.setAttribute('aria-pressed', String(theme === 'dark'));
  if (themeToggle) {
    themeToggle.setAttribute('aria-label', theme === 'dark' ? '라이트 모드로 전환' : '다크 모드로 전환');
  }
  if (themeLabel) themeLabel.textContent = theme === 'dark' ? '라이트 모드로 전환' : '다크 모드로 전환';
  if (persist) window.localStorage.setItem(themeStorageKey, theme);
}

function applyLanguage(code, persist = false) {
  root.lang = code;
  root.dataset.language = code;
  const lang = LANGUAGES.find((l) => l.code === code);

  langPickers.forEach((picker) => {
    const current = picker.querySelector('[data-lang-current]');
    const btn = picker.querySelector('[data-lang-btn]');
    if (current && lang) current.textContent = lang.short;
    if (btn && lang) btn.setAttribute('aria-label', `언어 선택 (현재: ${lang.label})`);
    picker.querySelectorAll('[data-lang-option]').forEach((opt) => {
      opt.setAttribute('aria-selected', String(opt.dataset.langOption === code));
    });
  });

  mobileLangRows.forEach((row) => {
    row.querySelectorAll('[data-lang-option]').forEach((btn) => {
      btn.setAttribute('aria-pressed', String(btn.dataset.langOption === code));
    });
  });

  if (persist) {
    document.cookie = `${languageStorageKey}=${code};path=/;max-age=31536000;SameSite=Lax`;
    window.localStorage.setItem(languageStorageKey, code);
    window.location.reload();
  }
}

function openLangPicker(picker) {
  const btn = picker.querySelector('[data-lang-btn]');
  const menu = picker.querySelector('[data-lang-menu]');
  if (!btn || !menu) return;
  btn.setAttribute('aria-expanded', 'true');
  menu.hidden = false;
}

function closeLangPicker(picker) {
  const btn = picker.querySelector('[data-lang-btn]');
  const menu = picker.querySelector('[data-lang-menu]');
  if (!btn || !menu) return;
  btn.setAttribute('aria-expanded', 'false');
  menu.hidden = true;
}

function closeAllLangPickers() {
  langPickers.forEach(closeLangPicker);
}

function updateHeader() {
  if (!header) return;
  header.classList.toggle('is-scrolled', header.hasAttribute('data-solid-header') || window.scrollY > 24);
}

function updateHeroParallax() {
  if (!heroBg || motionQuery.matches) return;
  const shift = Math.min(window.scrollY * 0.08, 34);
  heroBg.style.setProperty('--hero-shift', `${shift}px`);
}

function closeMobileMenu() {
  if (!menuButton || !mobileMenu) return;
  menuButton.setAttribute('aria-expanded', 'false');
  mobileMenu.hidden = true;
  document.body.classList.remove('menu-open');
  if (header) header.classList.remove('menu-active');
}

function openMobileMenu() {
  if (!menuButton || !mobileMenu) return;
  menuButton.setAttribute('aria-expanded', 'true');
  mobileMenu.hidden = false;
  document.body.classList.add('menu-open');
  if (header) header.classList.add('menu-active');
}

function triggerStagger(parent) {
  const children = Array.from(parent.querySelectorAll('[data-stagger]'));
  children.forEach((child, i) => {
    child.style.transitionDelay = `${120 + i * 160}ms`;
    child.classList.add('is-stagger-visible');
  });
}

function setupReveal() {
  if (!revealItems.length) return;
  if (motionQuery.matches || !('IntersectionObserver' in window)) {
    revealItems.forEach((item) => {
      item.classList.add('is-visible');
      item.querySelectorAll('[data-stagger]').forEach((c) => c.classList.add('is-stagger-visible'));
    });
    return;
  }

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (!entry.isIntersecting) return;
        entry.target.classList.add('is-visible');
        triggerStagger(entry.target);
        observer.unobserve(entry.target);
      });
    },
    { rootMargin: '0px 0px -4% 0px', threshold: 0.04 },
  );

  revealItems.forEach((item) => observer.observe(item));
}

const progressBar = document.createElement('div');
progressBar.className = 'scroll-progress';
document.body.prepend(progressBar);

function updateScrollProgress() {
  const maxScroll = document.documentElement.scrollHeight - window.innerHeight;
  if (maxScroll <= 0) return;
  progressBar.style.width = `${(window.scrollY / maxScroll) * 100}%`;
}

applyTheme(getStoredTheme() || (isProductPage ? 'light' : getSystemTheme()));
applyLanguage(getStoredLanguage() || 'ko');
setupReveal();
updateHeader();
updateHeroParallax();
updateScrollProgress();

window.addEventListener(
  'scroll',
  () => {
    updateHeader();
    updateHeroParallax();
    updateScrollProgress();
  },
  { passive: true },
);

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
  if (!getStoredTheme() && !isProductPage) applyTheme(getSystemTheme());
});

motionQuery.addEventListener('change', () => {
  if (motionQuery.matches && heroBg) heroBg.style.setProperty('--hero-shift', '0px');
});

if (themeToggle) {
  themeToggle.addEventListener('click', () => {
    const nextTheme = root.dataset.theme === 'dark' ? 'light' : 'dark';
    applyTheme(nextTheme, true);
  });
}

langPickers.forEach((picker) => {
  const btn = picker.querySelector('[data-lang-btn]');
  const menu = picker.querySelector('[data-lang-menu]');

  if (btn) {
    btn.addEventListener('click', (e) => {
      e.stopPropagation();
      const isOpen = btn.getAttribute('aria-expanded') === 'true';
      closeAllLangPickers();
      if (!isOpen) openLangPicker(picker);
    });
  }

  if (menu) {
    menu.querySelectorAll('[data-lang-option]').forEach((opt) => {
      opt.addEventListener('click', () => {
        applyLanguage(opt.dataset.langOption, true);
        closeLangPicker(picker);
      });
    });
  }
});

mobileLangRows.forEach((row) => {
  row.querySelectorAll('[data-lang-option]').forEach((btn) => {
    btn.addEventListener('click', () => {
      applyLanguage(btn.dataset.langOption, true);
    });
  });
});

document.addEventListener('click', closeAllLangPickers);
document.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') closeAllLangPickers();
});

if (menuButton && mobileMenu) {
  menuButton.addEventListener('click', () => {
    const expanded = menuButton.getAttribute('aria-expanded') === 'true';
    if (expanded) closeMobileMenu();
    else openMobileMenu();
  });

  mobileMenu.querySelectorAll('a').forEach((link) => {
    link.addEventListener('click', closeMobileMenu);
  });
}

(function setupCarousel() {
  const carousel = document.querySelector('[data-carousel]');
  if (!carousel) return;

  const track = carousel.querySelector('[data-carousel-track]');
  const slides = Array.from(track.querySelectorAll('.product-slide-card'));
  const dotsWrap = carousel.querySelector('[data-carousel-dots]');
  const prevBtn = carousel.querySelector('[data-carousel-prev]');
  const nextBtn = carousel.querySelector('[data-carousel-next]');

  if (!slides.length) return;

  let current = 0;
  let timer = null;
  const DELAY = 2500;

  const dots = slides.map((_, i) => {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'carousel-dot';
    btn.setAttribute('aria-label', `슬라이드 ${i + 1}`);
    btn.addEventListener('click', () => goTo(i));
    dotsWrap.appendChild(btn);
    return btn;
  });

  function goTo(index) {
    current = ((index % slides.length) + slides.length) % slides.length;
    track.style.transform = `translateX(-${current * 100}%)`;
    dots.forEach((d, i) => d.classList.toggle('is-active', i === current));
  }

  function startTimer() {
    clearInterval(timer);
    timer = setInterval(() => goTo(current + 1), DELAY);
  }

  function stopTimer() {
    clearInterval(timer);
    timer = null;
  }

  if (prevBtn) prevBtn.addEventListener('click', () => { goTo(current - 1); startTimer(); });
  if (nextBtn) nextBtn.addEventListener('click', () => { goTo(current + 1); startTimer(); });

  carousel.addEventListener('mouseenter', stopTimer);
  carousel.addEventListener('mouseleave', startTimer);
  carousel.addEventListener('focusin', stopTimer);
  carousel.addEventListener('focusout', startTimer);

  goTo(0);
  startTimer();
})();

(function setupZkpWhyExpand() {
  const section = document.querySelector('[data-zkp-why]');
  if (!section) return;
  const inner = section.querySelector('.zkp-why-inner');
  if (!inner) return;
  if (motionQuery.matches) return;

  let ticking = false;

  function update() {
    const rect = section.getBoundingClientRect();
    const vh = window.innerHeight;
    const start = vh * 1.1;
    const end = vh * 0.4;
    const progress = Math.max(0, Math.min(1, (start - rect.top) / (start - end)));

    const ease = progress < 0.5
      ? 2 * progress * progress
      : 1 - Math.pow(-2 * progress + 2, 2) / 2;

    const vMargin = (10 * (1 - ease)).toFixed(2);
    const hMargin = (25 * (1 - ease)).toFixed(2);
    const radius = Math.round(24 * (1 - ease));

    inner.style.borderRadius = `${radius}px`;
    inner.style.marginLeft   = `${hMargin}vw`;
    inner.style.marginRight  = `${hMargin}vw`;
    inner.style.marginTop    = `${vMargin}rem`;
    inner.style.marginBottom = `${vMargin}rem`;

    // 섹션 패딩도 함께 줄어들어 수직 확장감 강화
    const sectionPad = Math.round(80 + 80 * (1 - ease));
    section.style.paddingTop    = `${sectionPad}px`;
    section.style.paddingBottom = `${sectionPad}px`;

    ticking = false;
  }

  window.addEventListener('scroll', () => {
    if (!ticking) { requestAnimationFrame(update); ticking = true; }
  }, { passive: true });

  update();
})();
