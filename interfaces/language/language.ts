export interface ILanguageService {
  detectLanguage(text: string): Promise<Language>;
  translate(text: string, targetLang: string): Promise<string>;
}

export interface Language {
  code: string;
  name: string;
  region: string;
}

export class LanguageService implements ILanguageService {
  private supportedLanguages: Map<string, Language>;

  constructor(initialLanguages: Language[] = []) {
    this.supportedLanguages = new Map();
    initialLanguages.forEach((lang) => {
      this.supportedLanguages.set(lang.code, lang);
    });
  }

  public async detectLanguage(text: string): Promise<Language> {
    if (text.length === 0) {
      throw new Error('Text must not be empty');
    }
    const detectedCode = text.substring(0, 2).toLowerCase();
    const detected = this.supportedLanguages.get(detectedCode);
    if (!detected) {
      throw new Error(`Unsupported language: ${detectedCode}`);
    }
    return detected;
  }

  public async translate(text: string, targetLang: string): Promise<string> {
    if (!text.trim()) {
      throw new Error('Text must not be empty');
    }
    if (!this.supportedLanguages.has(targetLang)) {
      throw new Error(`Unsupported target language: ${targetLang}`);
    }
    return `[${targetLang}] ${text}`;
  }
}

export const createLanguageService = (
  languages: Language[] = []
): ILanguageService => {
  return new LanguageService(languages);
};
