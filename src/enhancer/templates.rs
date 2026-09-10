//! Web UI templates for the Prompt Enhancer
//! Based on Augment VSCode plugin official templates

/// Prompt enhancement template
/// Copied from augment.mjs YDn function - must stay in sync
/// Contains placeholder for original prompt only
pub const ENHANCE_PROMPT_TEMPLATE: &str = r#"⚠️ NO TOOLS ALLOWED ⚠️

Here is an instruction that I'd like to give you, but it needs to be improved. Rewrite and enhance this instruction to make it clearer, more specific, less ambiguous, and correct any mistakes. Do not use any tools: reply immediately with your answer, even if you're not sure. Consider the context of our conversation history when enhancing the prompt. If there is code in triple backticks (```) consider whether it is a code sample and should remain unchanged.Reply with the following format:

### BEGIN RESPONSE ###
Here is an enhanced version of the original instruction that is more specific and clear:
<augment-enhanced-prompt>enhanced prompt goes here</augment-enhanced-prompt>

### END RESPONSE ###

Here is my original instruction:

{original_prompt}"#;

/// Web UI HTML template for the Prompt Enhancer
pub const ENHANCER_UI_HTML: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Prompt Enhancer - BCE Tool</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Helvetica Neue', sans-serif;
      background: #f5f5f5;
      min-height: 100vh;
      padding: 20px;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .container {
      background: white;
      border-radius: 8px;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
      border: 1px solid #e0e0e0;
      max-width: 1000px;
      width: 100%;
      overflow: hidden;
    }

    .header {
      position: relative;
      background: white;
      color: #333;
      padding: 30px;
      text-align: center;
      border-bottom: 1px solid #e0e0e0;
    }

    .header h1 {
      font-size: 24px;
      font-weight: 600;
      margin-bottom: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10px;
      color: #333;
    }

    .header p {
      font-size: 14px;
      color: #666;
    }

    .lang-btn {
      position: absolute;
      top: 16px;
      right: 16px;
      padding: 6px 14px;
      border: 1px solid #ddd;
      border-radius: 6px;
      background: white;
      color: #555;
      font-size: 12px;
      font-weight: 600;
      box-shadow: none;
    }

    .lang-btn:hover {
      background: #f5f5f5;
      border-color: #bbb;
    }

    .countdown {
      margin-top: 12px;
      padding: 8px 16px;
      background: #f0f0f0;
      border-radius: 6px;
      display: inline-block;
      font-size: 13px;
      font-weight: 500;
      color: #555;
    }

    .countdown.warning {
      background: #fff3cd;
      color: #856404;
    }

    .countdown.danger {
      background: #f8d7da;
      color: #721c24;
      animation: pulse 1s ease-in-out infinite;
    }

    @keyframes pulse {
      0%, 100% { opacity: 1; }
      50% { opacity: 0.7; }
    }

    .content {
      padding: 30px;
    }

    .section {
      margin-bottom: 25px;
    }

    .section-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 10px;
    }

    .section-title {
      font-size: 14px;
      font-weight: 600;
      color: #333;
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }

    .view-tabs {
      display: flex;
      border: 1px solid #ddd;
      border-radius: 6px;
      overflow: hidden;
    }

    .tab-btn {
      padding: 5px 16px;
      border: none;
      border-radius: 0;
      background: white;
      color: #666;
      font-size: 12px;
      font-weight: 600;
      box-shadow: none;
    }

    .tab-btn + .tab-btn {
      border-left: 1px solid #ddd;
    }

    .tab-btn.active {
      background: #333;
      color: white;
    }

    .tab-btn:hover:not(.active) {
      background: #f5f5f5;
    }

    .editor-wrapper {
      position: relative;
    }

    textarea {
      width: 100%;
      min-height: 350px;
      padding: 16px;
      border: 2px solid #e0e0e0;
      border-radius: 8px;
      font-family: 'SF Mono', 'Monaco', 'Menlo', 'Consolas', monospace;
      font-size: 14px;
      line-height: 1.6;
      resize: vertical;
      transition: border-color 0.3s;
      background: #fafafa;
    }

    textarea:focus {
      outline: none;
      border-color: #333;
      background: white;
    }

    .markdown-preview {
      width: 100%;
      min-height: 350px;
      max-height: 600px;
      overflow-y: auto;
      padding: 16px;
      border: 2px solid #e0e0e0;
      border-radius: 8px;
      background: #fafafa;
      font-size: 14px;
      line-height: 1.7;
      color: #333;
      word-break: break-word;
    }

    .markdown-preview > :first-child {
      margin-top: 0;
    }

    .markdown-preview h1 { font-size: 22px; margin: 16px 0 10px; }
    .markdown-preview h2 { font-size: 19px; margin: 14px 0 8px; }
    .markdown-preview h3 { font-size: 16px; margin: 12px 0 8px; }
    .markdown-preview h4,
    .markdown-preview h5,
    .markdown-preview h6 { font-size: 14px; margin: 10px 0 6px; }

    .markdown-preview p {
      margin: 8px 0;
    }

    .markdown-preview ul,
    .markdown-preview ol {
      margin: 8px 0;
      padding-left: 24px;
    }

    .markdown-preview li {
      margin: 4px 0;
    }

    .markdown-preview code {
      background: #ececec;
      padding: 2px 5px;
      border-radius: 4px;
      font-family: 'SF Mono', 'Monaco', 'Menlo', 'Consolas', monospace;
      font-size: 13px;
    }

    .markdown-preview pre {
      background: #2d2d2d;
      color: #f0f0f0;
      padding: 14px;
      border-radius: 6px;
      overflow-x: auto;
      margin: 10px 0;
    }

    .markdown-preview pre code {
      background: none;
      color: inherit;
      padding: 0;
      font-size: 13px;
      line-height: 1.5;
    }

    .markdown-preview blockquote {
      border-left: 4px solid #ccc;
      padding: 4px 12px;
      color: #666;
      margin: 10px 0;
      background: #f5f5f5;
    }

    .markdown-preview table {
      border-collapse: collapse;
      margin: 10px 0;
      width: 100%;
    }

    .markdown-preview th,
    .markdown-preview td {
      border: 1px solid #ddd;
      padding: 6px 10px;
      text-align: left;
    }

    .markdown-preview th {
      background: #f5f5f5;
    }

    .markdown-preview img {
      max-width: 100%;
    }

    .markdown-preview hr {
      border: none;
      border-top: 1px solid #ddd;
      margin: 14px 0;
    }

    .markdown-preview a {
      color: #0366d6;
    }

    .char-count {
      position: absolute;
      bottom: 12px;
      right: 12px;
      background: rgba(255, 255, 255, 0.9);
      padding: 4px 10px;
      border-radius: 12px;
      font-size: 12px;
      color: #666;
      pointer-events: none;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .info-box {
      background: #f9f9f9;
      border-left: 4px solid #333;
      padding: 15px;
      border-radius: 4px;
      margin-bottom: 20px;
    }

    .info-box p {
      font-size: 14px;
      color: #555;
      line-height: 1.6;
    }

    .buttons {
      display: flex;
      gap: 12px;
      justify-content: flex-end;
      margin-top: 25px;
    }

    button {
      padding: 12px 28px;
      border: none;
      border-radius: 8px;
      font-size: 15px;
      font-weight: 600;
      cursor: pointer;
      transition: all 0.3s;
      display: flex;
      align-items: center;
      gap: 8px;
    }

    .send-btn {
      background: #333;
      color: white;
      box-shadow: none;
    }

    .send-btn:hover:not(:disabled) {
      background: #000;
    }

    .send-btn:active:not(:disabled) {
      background: #000;
    }

    .send-btn:disabled {
      background: #ccc;
      cursor: not-allowed;
      box-shadow: none;
    }

    .cancel-btn {
      background: white;
      color: #666;
      border: 2px solid #e0e0e0;
    }

    .cancel-btn:hover {
      background: #f5f5f5;
      border-color: #ccc;
    }

    .re-enhance-btn {
      background: white;
      color: #333;
      border: 2px solid #333;
    }

    .re-enhance-btn:hover:not(:disabled) {
      background: #f5f5f5;
      border-color: #000;
    }

    .re-enhance-btn:disabled {
      background: #f5f5f5;
      color: #ccc;
      border-color: #e0e0e0;
      cursor: not-allowed;
    }

    .status {
      margin-top: 20px;
      padding: 15px;
      border-radius: 8px;
      display: none;
      animation: slideIn 0.3s ease;
    }

    @keyframes slideIn {
      from {
        opacity: 0;
        transform: translateY(-10px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }

    .status.success {
      background: #d4edda;
      color: #155724;
      border-left: 4px solid #28a745;
      display: block;
    }

    .status.error {
      background: #f8d7da;
      color: #721c24;
      border-left: 4px solid #dc3545;
      display: block;
    }

    .loading {
      display: none;
      text-align: center;
      padding: 40px;
    }

    .loading.active {
      display: block;
    }

    .spinner {
      border: 3px solid #f3f3f3;
      border-top: 3px solid #333;
      border-radius: 50%;
      width: 40px;
      height: 40px;
      animation: spin 1s linear infinite;
      margin: 0 auto 15px;
    }

    @keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }

    .keyboard-hint {
      font-size: 12px;
      color: #999;
      text-align: center;
      margin-top: 15px;
    }

    .keyboard-hint kbd {
      background: #f5f5f5;
      border: 1px solid #ddd;
      border-radius: 4px;
      padding: 2px 6px;
      font-family: monospace;
      font-size: 11px;
    }

    @media (max-width: 768px) {
      body {
        padding: 10px;
      }

      .header {
        padding: 20px;
      }

      .header h1 {
        font-size: 22px;
      }

      .content {
        padding: 20px;
      }

      textarea {
        min-height: 250px;
        font-size: 13px;
      }

      .markdown-preview {
        min-height: 250px;
        font-size: 13px;
      }

      .buttons {
        flex-direction: column-reverse;
      }

      .buttons button {
        width: 100%;
        justify-content: center;
      }
    }
  </style>
</head>
<body>
  <div class="container">
    <div class="header">
      <button type="button" class="lang-btn" id="langBtn" onclick="toggleLang()">EN</button>
      <h1>
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2L2 7l10 5 10-5-10-5z"/>
          <path d="M2 17l10 5 10-5"/>
          <path d="M2 12l10 5 10-5"/>
        </svg>
        Prompt Enhancer
      </h1>
      <p id="subtitle">查看并完善增强后的提示词</p>
      <div class="countdown" id="countdown">加载中...</div>
    </div>

    <div class="content">
      <div class="loading" id="loading">
        <div class="spinner"></div>
        <p id="loadingText">正在加载增强后的提示词...</p>
      </div>

      <div id="mainContent" style="display: none;">
        <div class="info-box">
          <p id="tipText"></p>
        </div>

        <div class="section">
          <div class="section-header">
            <div class="section-title" id="sectionTitle">增强后的提示词</div>
            <div class="view-tabs">
              <button type="button" class="tab-btn" id="tabPreview" onclick="setView('preview')">预览</button>
              <button type="button" class="tab-btn active" id="tabEdit" onclick="setView('edit')">编辑</button>
            </div>
          </div>
          <div class="editor-wrapper">
            <div class="markdown-preview" id="markdownPreview" style="display: none;" ondblclick="setView('edit')"></div>
            <textarea
              id="promptText"
              placeholder="增强后的提示词将显示在这里..."
              spellcheck="false"
            ></textarea>
            <div class="char-count" id="charCount">0 字</div>
          </div>
        </div>

        <div class="buttons">
          <button class="cancel-btn" onclick="endConversation()">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
            <span id="endText">结束对话</span>
          </button>
          <button class="re-enhance-btn" id="reEnhanceBtn" onclick="reEnhance()">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="23 4 23 10 17 10"/>
              <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
            </svg>
            <span id="reEnhanceText">重新增强</span>
          </button>
          <button class="cancel-btn" onclick="useOriginal()">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 12h18M3 6h18M3 18h18"/>
            </svg>
            <span id="useOriginalText">使用原始提示词</span>
          </button>
          <button class="send-btn" id="sendBtn" onclick="sendPrompt()">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="22" y1="2" x2="11" y2="13"/>
              <polygon points="22 2 15 22 11 13 2 9 22 2"/>
            </svg>
            <span id="sendText">发送增强提示词</span>
          </button>
        </div>

        <div class="keyboard-hint" id="keyboardHint"></div>

        <div id="status" class="status"></div>
      </div>
    </div>
  </div>

  <script>
    const I18N = {
      zh: {
        langBtn: 'EN',
        subtitle: '查看并完善增强后的提示词',
        loadingCountdown: '加载中...',
        loadingText: '正在加载增强后的提示词...',
        tip: '<strong>提示：</strong>AI 已根据对话历史和代码上下文增强了你的提示词。你可以在下方继续编辑，然后点击“发送增强提示词”继续。',
        sectionTitle: '增强后的提示词',
        tabPreview: '预览',
        tabEdit: '编辑',
        placeholder: '增强后的提示词将显示在这里...',
        chars: ' 字',
        emptyPreview: '（暂无内容）',
        btnEnd: '结束对话',
        btnReEnhance: '重新增强',
        btnUseOriginal: '使用原始提示词',
        btnSend: '发送增强提示词',
        hint: '快捷键：<kbd>Ctrl</kbd> + <kbd>Enter</kbd> 发送 | <kbd>Esc</kbd> 结束对话',
        remaining: '剩余时间: ',
        timedOut: '已超时',
        errNoSession: '错误：缺少会话 ID',
        loadFailed: '加载失败: ',
        emptyContent: '请先输入内容',
        enhancing: '增强中...',
        sending: '发送中...',
        enhanceOk: '增强成功！你可以继续编辑或发送。',
        enhanceFail: '增强失败: ',
        sendOk: '发送成功！窗口将在 2 秒后关闭...',
        sendFail: '发送失败: ',
        confirmOriginal: '确定要使用原始提示词吗？',
        useOriginalOk: '将使用原始提示词...',
        confirmEnd: '确定要结束本次对话吗？',
        endOk: '对话已结束',
        failed: '操作失败: '
      },
      en: {
        langBtn: '中文',
        subtitle: 'Review and refine your enhanced prompt',
        loadingCountdown: 'Loading...',
        loadingText: 'Loading your enhanced prompt...',
        tip: '<strong>Tip:</strong> AI has enhanced your prompt based on conversation history and code context. You can further edit it below, then click Send Enhanced to continue.',
        sectionTitle: 'Enhanced Prompt',
        tabPreview: 'Preview',
        tabEdit: 'Edit',
        placeholder: 'Your enhanced prompt will appear here...',
        chars: ' chars',
        emptyPreview: '(No content)',
        btnEnd: 'End Chat',
        btnReEnhance: 'Re-enhance',
        btnUseOriginal: 'Use Original',
        btnSend: 'Send Enhanced',
        hint: 'Shortcuts: <kbd>Ctrl</kbd> + <kbd>Enter</kbd> Send | <kbd>Esc</kbd> End Chat',
        remaining: 'Remaining: ',
        timedOut: 'Timed out',
        errNoSession: 'Error: No session ID provided',
        loadFailed: 'Load failed: ',
        emptyContent: 'Please enter content first',
        enhancing: 'Enhancing...',
        sending: 'Sending...',
        enhanceOk: 'Enhancement successful! You can continue editing or send.',
        enhanceFail: 'Enhancement failed: ',
        sendOk: 'Sent successfully! Window will close in 2 seconds...',
        sendFail: 'Send failed: ',
        confirmOriginal: 'Are you sure you want to use the original prompt?',
        useOriginalOk: 'Will use original prompt...',
        confirmEnd: 'Are you sure you want to end this conversation?',
        endOk: 'Conversation ended',
        failed: 'Failed: '
      }
    };

    let lang = 'zh';
    try {
      const saved = localStorage.getItem('bce-enhancer-lang');
      if (saved === 'zh' || saved === 'en') lang = saved;
    } catch (e) {}

    function t(key) {
      return (I18N[lang] && I18N[lang][key]) || I18N.zh[key] || key;
    }

    const urlParams = new URLSearchParams(window.location.search);
    const sessionId = urlParams.get('session');
    const promptText = document.getElementById('promptText');
    const charCount = document.getElementById('charCount');
    const loading = document.getElementById('loading');
    const mainContent = document.getElementById('mainContent');
    const countdownEl = document.getElementById('countdown');
    const markdownPreview = document.getElementById('markdownPreview');

    let countdownInterval = null;
    let sessionCreatedAt = null;
    let sessionTimeoutMs = null;
    let viewMode = 'edit';

    const SVG_RE_ENHANCE = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/></svg>';
    const SVG_SEND = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="22" y1="2" x2="11" y2="13"/><polygon points="22 2 15 22 11 13 2 9 22 2"/></svg>';

    function restoreReEnhanceBtn() {
      document.getElementById('reEnhanceBtn').innerHTML = SVG_RE_ENHANCE + '<span id="reEnhanceText">' + t('btnReEnhance') + '</span>';
    }

    function restoreSendBtn() {
      document.getElementById('sendBtn').innerHTML = SVG_SEND + '<span id="sendText">' + t('btnSend') + '</span>';
    }

    function applyLang() {
      document.documentElement.lang = lang === 'zh' ? 'zh-CN' : 'en';
      document.getElementById('langBtn').textContent = t('langBtn');
      document.getElementById('subtitle').textContent = t('subtitle');
      document.getElementById('loadingText').textContent = t('loadingText');
      document.getElementById('tipText').innerHTML = t('tip');
      document.getElementById('sectionTitle').textContent = t('sectionTitle');
      document.getElementById('tabPreview').textContent = t('tabPreview');
      document.getElementById('tabEdit').textContent = t('tabEdit');
      promptText.placeholder = t('placeholder');
      document.getElementById('endText').textContent = t('btnEnd');
      const reText = document.getElementById('reEnhanceText');
      if (reText) reText.textContent = t('btnReEnhance');
      document.getElementById('useOriginalText').textContent = t('btnUseOriginal');
      const sendText = document.getElementById('sendText');
      if (sendText) sendText.textContent = t('btnSend');
      document.getElementById('keyboardHint').innerHTML = t('hint');
      updateCharCount();
      if (sessionCreatedAt && sessionTimeoutMs) {
        updateCountdown();
      } else {
        countdownEl.textContent = t('loadingCountdown');
      }
      if (viewMode === 'preview') {
        renderPreview();
      }
    }

    function toggleLang() {
      lang = lang === 'zh' ? 'en' : 'zh';
      try { localStorage.setItem('bce-enhancer-lang', lang); } catch (e) {}
      applyLang();
    }

    // ===== Markdown rendering =====

    function escapeHtml(s) {
      return s
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
    }

    function mdInline(s) {
      return s
        .replace(/`([^`]+)`/g, '<code>$1</code>')
        .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
        .replace(/__([^_]+)__/g, '<strong>$1</strong>')
        .replace(/(^|[^*])\*([^*\n]+)\*/g, '$1<em>$2</em>')
        .replace(/~~([^~]+)~~/g, '<del>$1</del>')
        .replace(/!\[([^\]]*)\]\(([^)\s]+)\)/g, '<img alt="$1" src="$2">')
        .replace(/\[([^\]]+)\]\(([^)\s]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>');
    }

    function buildList(items) {
      let out = '';
      const stack = [];
      items.forEach(function(it) {
        const level = Math.floor(it.indent / 2);
        if (stack.length === 0 || level > stack.length - 1) {
          const tag = it.ordered ? 'ol' : 'ul';
          out += '<' + tag + '>';
          stack.push(tag);
        } else {
          while (stack.length - 1 > level && stack.length > 1) {
            out += '</li></' + stack.pop() + '>';
          }
          out += '</li>';
        }
        out += '<li>' + mdInline(it.text);
      });
      while (stack.length) {
        out += '</li></' + stack.pop() + '>';
      }
      return out;
    }

    function renderMarkdown(src) {
      if (!src || !src.trim()) return '';

      const codeBlocks = [];
      let text = src.replace(/\r\n/g, '\n');

      // Extract fenced code blocks first so their content stays verbatim
      text = text.replace(/```[^\n]*\n([\s\S]*?)(?:\n```|$)/g, function(m, code) {
        codeBlocks.push('<pre><code>' + escapeHtml(code) + '</code></pre>');
        return '\u0000MDB' + (codeBlocks.length - 1) + '\u0000';
      });

      text = escapeHtml(text);

      const lines = text.split('\n');
      const out = [];
      let para = [];
      let i = 0;

      function flushPara() {
        if (para.length) {
          out.push('<p>' + para.join('<br>') + '</p>');
          para = [];
        }
      }

      function isTableSep(s) {
        return s.indexOf('-') !== -1 && /^[\s|:\-]+$/.test(s);
      }

      function parseRow(s) {
        return s.replace(/^\s*\|/, '').replace(/\|\s*$/, '').split('|').map(function(c) {
          return c.trim();
        });
      }

      while (i < lines.length) {
        const line = lines[i];
        let m;

        if (!line.trim()) {
          flushPara();
          i++;
          continue;
        }

        if ((m = line.match(/^\u0000MDB(\d+)\u0000\s*$/))) {
          flushPara();
          out.push(codeBlocks[+m[1]]);
          i++;
          continue;
        }

        if ((m = line.match(/^(#{1,6})\s+(.+)$/))) {
          flushPara();
          const lv = m[1].length;
          out.push('<h' + lv + '>' + mdInline(m[2]) + '</h' + lv + '>');
          i++;
          continue;
        }

        if (/^\s*(-{3,}|\*{3,}|_{3,})\s*$/.test(line)) {
          flushPara();
          out.push('<hr>');
          i++;
          continue;
        }

        if (/^\s*&gt;/.test(line)) {
          flushPara();
          const quote = [];
          while (i < lines.length && /^\s*&gt;/.test(lines[i])) {
            quote.push(mdInline(lines[i].replace(/^\s*&gt;\s?/, '')));
            i++;
          }
          out.push('<blockquote>' + quote.join('<br>') + '</blockquote>');
          continue;
        }

        if (/^\s*([-*+]|\d+[.)])\s+/.test(line)) {
          flushPara();
          const items = [];
          while (i < lines.length && /^\s*([-*+]|\d+[.)])\s+/.test(lines[i])) {
            const lm = lines[i].match(/^(\s*)([-*+]|\d+[.)])\s+(.*)$/);
            items.push({
              indent: lm[1].length,
              ordered: /^\d/.test(lm[2]),
              text: lm[3]
            });
            i++;
          }
          out.push(buildList(items));
          continue;
        }

        if (line.indexOf('|') !== -1 && i + 1 < lines.length && isTableSep(lines[i + 1])) {
          flushPara();
          let table = '<table><thead><tr>';
          parseRow(line).forEach(function(h) {
            table += '<th>' + mdInline(h) + '</th>';
          });
          table += '</tr></thead><tbody>';
          i += 2;
          while (i < lines.length && lines[i].indexOf('|') !== -1) {
            table += '<tr>';
            parseRow(lines[i]).forEach(function(c) {
              table += '<td>' + mdInline(c) + '</td>';
            });
            table += '</tr>';
            i++;
          }
          out.push(table + '</tbody></table>');
          continue;
        }

        para.push(mdInline(line));
        i++;
      }
      flushPara();

      let html = out.join('\n');
      // Restore any placeholders that ended up inside other blocks
      html = html.replace(/\u0000MDB(\d+)\u0000/g, function(mm, n) {
        return codeBlocks[+n];
      });
      return html;
    }

    function renderPreview() {
      const html = renderMarkdown(promptText.value);
      markdownPreview.innerHTML = html || '<p style="color: #999;">' + t('emptyPreview') + '</p>';
    }

    function setView(mode) {
      viewMode = mode;
      const tabPreview = document.getElementById('tabPreview');
      const tabEdit = document.getElementById('tabEdit');
      if (mode === 'preview') {
        renderPreview();
        markdownPreview.style.display = 'block';
        promptText.style.display = 'none';
        tabPreview.classList.add('active');
        tabEdit.classList.remove('active');
      } else {
        markdownPreview.style.display = 'none';
        promptText.style.display = 'block';
        tabEdit.classList.add('active');
        tabPreview.classList.remove('active');
        promptText.focus();
      }
    }

    // ===== Character count =====

    function updateCharCount() {
      const count = promptText.value.length;
      charCount.textContent = count + t('chars');
    }

    promptText.addEventListener('input', updateCharCount);

    // ===== Countdown =====

    function formatTime(ms) {
      const totalSeconds = Math.floor(ms / 1000);
      const minutes = Math.floor(totalSeconds / 60);
      const seconds = totalSeconds % 60;
      return minutes + ':' + seconds.toString().padStart(2, '0');
    }

    function updateCountdown() {
      if (!sessionCreatedAt || !sessionTimeoutMs) return;

      const now = Date.now();
      const elapsed = now - sessionCreatedAt;
      const remaining = sessionTimeoutMs - elapsed;

      if (remaining <= 0) {
        countdownEl.textContent = t('timedOut');
        countdownEl.className = 'countdown danger';
        if (countdownInterval) {
          clearInterval(countdownInterval);
          countdownInterval = null;
        }
        return;
      }

      const remainingMinutes = remaining / 60000;

      if (remainingMinutes <= 1) {
        countdownEl.className = 'countdown danger';
      } else if (remainingMinutes <= 3) {
        countdownEl.className = 'countdown warning';
      } else {
        countdownEl.className = 'countdown';
      }

      countdownEl.textContent = t('remaining') + formatTime(remaining);
    }

    function startCountdown(createdAt, timeoutMs) {
      sessionCreatedAt = createdAt;
      sessionTimeoutMs = timeoutMs;

      updateCountdown();

      if (countdownInterval) {
        clearInterval(countdownInterval);
      }

      countdownInterval = setInterval(updateCountdown, 1000);
    }

    // ===== Keyboard shortcuts =====

    document.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        e.preventDefault();
        sendPrompt();
      } else if (e.key === 'Escape') {
        e.preventDefault();
        endConversation();
      }
    });

    // ===== Init =====

    applyLang();

    if (!sessionId) {
      loading.style.display = 'none';
      mainContent.style.display = 'block';
      showStatus(t('errNoSession'), 'error');
    } else {
      loading.classList.add('active');

      fetch('/api/session?session=' + encodeURIComponent(sessionId))
        .then(r => r.json())
        .then(data => {
          if (data.error) {
            throw new Error(data.error);
          }

          promptText.value = data.enhancedPrompt;
          updateCharCount();
          loading.classList.remove('active');
          mainContent.style.display = 'block';
          setView('preview');

          if (data.createdAt && data.timeoutMs) {
            startCountdown(data.createdAt, data.timeoutMs);
          }
        })
        .catch(err => {
          loading.classList.remove('active');
          mainContent.style.display = 'block';
          showStatus(t('loadFailed') + err.message, 'error');
        });
    }

    // ===== Actions =====

    function reEnhance() {
      const currentContent = promptText.value.trim();

      if (!currentContent) {
        showStatus(t('emptyContent'), 'error');
        return;
      }

      const reEnhanceBtn = document.getElementById('reEnhanceBtn');
      const sendBtn = document.getElementById('sendBtn');

      reEnhanceBtn.disabled = true;
      sendBtn.disabled = true;
      reEnhanceBtn.innerHTML = '<div class="spinner" style="width: 16px; height: 16px; border-width: 2px; margin: 0;"></div> ' + t('enhancing');

      fetch('/api/re-enhance', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          sessionId: sessionId,
          currentPrompt: currentContent
        })
      })
      .then(r => r.json())
      .then(data => {
        if (data.error) {
          throw new Error(data.error);
        }

        promptText.value = data.enhancedPrompt;
        updateCharCount();
        if (viewMode === 'preview') {
          renderPreview();
        }
        showStatus(t('enhanceOk'), 'success');

        reEnhanceBtn.disabled = false;
        sendBtn.disabled = false;
        restoreReEnhanceBtn();
      })
      .catch(err => {
        showStatus(t('enhanceFail') + err.message, 'error');
        reEnhanceBtn.disabled = false;
        sendBtn.disabled = false;
        restoreReEnhanceBtn();
      });
    }

    function sendPrompt() {
      const content = promptText.value.trim();

      if (!content) {
        showStatus(t('emptyContent'), 'error');
        return;
      }

      const sendBtn = document.getElementById('sendBtn');
      const reEnhanceBtn = document.getElementById('reEnhanceBtn');

      sendBtn.disabled = true;
      reEnhanceBtn.disabled = true;
      sendBtn.innerHTML = '<div class="spinner" style="width: 16px; height: 16px; border-width: 2px; margin: 0;"></div> ' + t('sending');

      fetch('/api/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ sessionId: sessionId, content: content, action: 'send' })
      })
      .then(r => r.json())
      .then(data => {
        if (data.error) {
          throw new Error(data.error);
        }
        showStatus(t('sendOk'), 'success');
        setTimeout(() => window.close(), 2000);
      })
      .catch(err => {
        showStatus(t('sendFail') + err.message, 'error');
        sendBtn.disabled = false;
        reEnhanceBtn.disabled = false;
        restoreSendBtn();
      });
    }

    function useOriginal() {
      if (confirm(t('confirmOriginal'))) {
        fetch('/api/submit', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ sessionId: sessionId, content: '', action: 'use_original' })
        })
        .then(r => r.json())
        .then(data => {
          if (data.error) {
            throw new Error(data.error);
          }
          showStatus(t('useOriginalOk'), 'success');
          setTimeout(() => window.close(), 1000);
        })
        .catch(err => {
          showStatus(t('failed') + err.message, 'error');
        });
      }
    }

    function endConversation() {
      if (confirm(t('confirmEnd'))) {
        fetch('/api/submit', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ sessionId: sessionId, content: '', action: 'end_conversation' })
        })
        .then(r => r.json())
        .then(data => {
          if (data.error) {
            throw new Error(data.error);
          }
          showStatus(t('endOk'), 'success');
          setTimeout(() => window.close(), 1000);
        })
        .catch(err => {
          showStatus(t('failed') + err.message, 'error');
        });
      }
    }

    function showStatus(message, type) {
      const status = document.getElementById('status');
      status.textContent = message;
      status.className = 'status ' + type;
    }
  </script>
</body>
</html>"#;
