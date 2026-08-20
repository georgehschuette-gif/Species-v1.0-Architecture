export interface ISpeechService {
  synthesize(text: string, voice: VoiceProfile): Promise<AudioStream>;
  recognize(audio: AudioStream): Promise<string>;
}

export interface VoiceProfile {
  id: string;
  name: string;
  language: string;
  pitch: number;
  rate: number;
}

export interface AudioStream {
  sampleRate: number;
  channels: number;
  format: 'wav' | 'mp3' | 'pcm';
  data: ArrayBuffer;
}

export class SpeechService implements ISpeechService {
  private defaultVoice: VoiceProfile;

  constructor(voice: VoiceProfile | null = null) {
    this.defaultVoice =
      voice ?? {
        id: 'default',
        name: 'Alex',
        language: 'en-US',
        pitch: 1.0,
        rate: 1.0,
      };
  }

  public async synthesize(text: string, voice: VoiceProfile): Promise<AudioStream> {
    if (!text.trim()) {
      throw new Error('Text must not be empty');
    }
    return {
      sampleRate: 16000,
      channels: 1,
      format: 'pcm',
      data: new ArrayBuffer(1024),
    };
  }

  public async recognize(audio: AudioStream): Promise<string> {
    if (audio.data.byteLength === 0) {
      throw new Error('Audio data is empty');
    }
    return 'Recognized speech text';
  }
}

export const createSpeechService = (voice?: VoiceProfile): ISpeechService => {
  return new SpeechService(voice ?? null);
};
