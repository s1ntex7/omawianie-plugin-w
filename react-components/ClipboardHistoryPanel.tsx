// ============================================================================
// CLIPBOARD HISTORY 2.0 - React Component
// ============================================================================
// Purpose: Popup UI for Ctrl+Shift+V - search and paste clipboard history
// ============================================================================

import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import toast from 'react-hot-toast';

interface ClipboardEntry {
  id: number;
  content_type: 'text' | 'image';
  content_text?: string;
  timestamp: number;
  pinned: boolean;
  label?: string;
}

export const ClipboardHistoryPanel: React.FC = () => {
  const [entries, setEntries] = useState<ClipboardEntry[]>([]);
  const [query, setQuery] = useState('');
  const [isOpen, setIsOpen] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  // Listen for Ctrl+Shift+V event from backend
  useEffect(() => {
    const unlisten = listen('clipboard-history:show-popup', () => {
      setIsOpen(true);
      loadRecent();
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  // Load recent entries
  const loadRecent = async () => {
    try {
      const result = await invoke<ClipboardEntry[]>('clipboard_history_recent', { limit: 50 });
      setEntries(result);
      setSelectedIndex(0);
    } catch (err) {
      toast.error(`Failed to load history: ${err}`);
    }
  };

  // Search entries
  const search = async (q: string) => {
    setQuery(q);

    if (!q) {
      await loadRecent();
      return;
    }

    try {
      const result = await invoke<ClipboardEntry[]>('clipboard_history_search', { query: q });
      setEntries(result);
      setSelectedIndex(0);
    } catch (err) {
      toast.error(`Search failed: ${err}`);
    }
  };

  // Paste entry
  const pasteEntry = async (entryId: number) => {
    try {
      await invoke('clipboard_history_paste', { entryId });
      setIsOpen(false);
      setQuery('');
      toast.success('Pasted!');
    } catch (err) {
      toast.error(`Failed to paste: ${err}`);
    }
  };

  // Toggle pin
  const togglePin = async (entryId: number, e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      const isPinned = await invoke<boolean>('clipboard_history_toggle_pin', { entryId });
      toast.success(isPinned ? 'Pinned!' : 'Unpinned!');
      search(query); // Refresh list
    } catch (err) {
      toast.error(`Failed to toggle pin: ${err}`);
    }
  };

  // Keyboard navigation
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      setIsOpen(false);
      setQuery('');
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex(prev => Math.min(prev + 1, entries.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex(prev => Math.max(prev - 1, 0));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (entries[selectedIndex]) {
        pasteEntry(entries[selectedIndex].id);
      }
    }
  };

  // Auto-focus search input when popup opens
  useEffect(() => {
    if (isOpen && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <div className="clipboard-history-popup" onKeyDown={handleKeyDown}>
      <div className="popup-overlay" onClick={() => setIsOpen(false)} />

      <div className="popup-content">
        <input
          ref={inputRef}
          type="text"
          placeholder="Search clipboard history..."
          value={query}
          onChange={(e) => search(e.target.value)}
          className="search-input"
        />

        <div className="entries-list">
          {entries.length === 0 ? (
            <div className="no-results">No entries found</div>
          ) : (
            entries.map((entry, index) => (
              <div
                key={entry.id}
                className={`entry ${index === selectedIndex ? 'selected' : ''} ${entry.pinned ? 'pinned' : ''}`}
                onClick={() => pasteEntry(entry.id)}
              >
                <div className="entry-content">
                  {entry.content_type === 'text' ? (
                    <p className="text-preview">
                      {entry.content_text?.substring(0, 100)}
                      {(entry.content_text?.length ?? 0) > 100 && '...'}
                    </p>
                  ) : (
                    <div className="image-preview">
                      <span>📷 Image</span>
                    </div>
                  )}
                </div>

                <div className="entry-meta">
                  <span className="timestamp">
                    {new Date(entry.timestamp * 1000).toLocaleTimeString()}
                  </span>
                  <button
                    className="pin-button"
                    onClick={(e) => togglePin(entry.id, e)}
                  >
                    {entry.pinned ? '📌' : '📍'}
                  </button>
                </div>
              </div>
            ))
          )}
        </div>

        <div className="popup-footer">
          <span>↑↓ Navigate</span>
          <span>Enter: Paste</span>
          <span>ESC: Close</span>
        </div>
      </div>

      <style>{`
        .clipboard-history-popup {
          position: fixed;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          z-index: 9999;
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .popup-overlay {
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background: rgba(0, 0, 0, 0.5);
        }

        .popup-content {
          position: relative;
          background: #2a2a2a;
          border-radius: 8px;
          width: 600px;
          max-height: 500px;
          display: flex;
          flex-direction: column;
          box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
        }

        .search-input {
          padding: 12px;
          border: none;
          border-bottom: 1px solid #444;
          background: #333;
          color: white;
          font-size: 16px;
          outline: none;
        }

        .entries-list {
          flex: 1;
          overflow-y: auto;
          padding: 8px;
        }

        .entry {
          padding: 12px;
          margin-bottom: 4px;
          background: #333;
          border-radius: 4px;
          cursor: pointer;
          display: flex;
          justify-content: space-between;
          align-items: center;
          transition: background 0.2s;
        }

        .entry:hover, .entry.selected {
          background: #444;
        }

        .entry.pinned {
          border-left: 3px solid #ffa500;
        }

        .entry-content {
          flex: 1;
          overflow: hidden;
        }

        .text-preview {
          margin: 0;
          color: #ddd;
          font-size: 14px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }

        .image-preview {
          color: #888;
          font-style: italic;
        }

        .entry-meta {
          display: flex;
          align-items: center;
          gap: 8px;
          margin-left: 12px;
        }

        .timestamp {
          color: #888;
          font-size: 12px;
        }

        .pin-button {
          background: none;
          border: none;
          cursor: pointer;
          font-size: 16px;
          opacity: 0.6;
          transition: opacity 0.2s;
        }

        .pin-button:hover {
          opacity: 1;
        }

        .popup-footer {
          padding: 8px 12px;
          background: #222;
          border-top: 1px solid #444;
          display: flex;
          justify-content: space-around;
          font-size: 12px;
          color: #888;
        }

        .no-results {
          text-align: center;
          padding: 40px;
          color: #888;
        }
      `}</style>
    </div>
  );
};
