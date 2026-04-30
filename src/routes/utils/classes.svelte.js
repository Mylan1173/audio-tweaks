// @ts-nocheck
import { LANGUAGES, AUDIO_CODECS } from "./maps.js";

export class MediaData {
  initialized = $state(false);
  path = $state();
  name = $state();

  _oldVideo = $state({});
  _oldAudio = $state([]);
  _oldSubtitle = $state([]);

  _video = $state({});
  _audio = $state([]);
  _subtitle = $state([]);

  init(rawData) {
    if (this.initialized) return;

    this.initialized = true;

    //Video Changes
    const rawVideo =
      rawData.streams.find((s) => s.codec_type === "video") || {};

    let video = {};
    if (rawVideo.codec_type) {
      video = {
        width: rawVideo.width,
        height: rawVideo.height,
        codecName: rawVideo.codec_name,
        aspectRatio: rawVideo.display_aspect_ratio,
        pixelFormat: rawVideo.pix_fmt,
        fieldOrder: rawVideo.field_order,
        profile: rawVideo.profile,
        avgFrameRate: rawVideo.avg_frame_rate,
      };
    }

    this._oldVideo = video;
    this._video = structuredClone(video);

    //Audio Changes
    const rawAudio = rawData.streams.filter((s) => s.codec_type === "audio");
    let audioArray = [];

    for (const audio of rawAudio) {
      let channelMap = {};
      let channelVolumes = {};
      for (let i = 0; i < audio.channels; i++) {
        channelMap[i] = i;
        channelVolumes[i] = 0;
      }

      audioArray.push({
        default: audio.disposition?.default === 1,
        forced: audio.disposition?.forced === 1,
        title: audio.tags?.title || "",
        language: audio.tags?.language || "und",
        isDeleted: false,
        isImported: false,
        path: null,
        bitRate: audio.bit_rate,
        bitDepth: audio.bits_per_sample,
        channels: audio.channels,
        newChannels: audio.channels,
        codecName: audio.codec_name,
        sampleRate: audio.sample_rate,
        channelMap,
        channelVolumes,
      });
    }

    this._oldAudio = audioArray;
    this._audio = structuredClone(audioArray);

    //Subtitle Changes
    const rawSubtitle = rawData.streams.filter(
      (s) => s.codec_type === "subtitle",
    );
    let subtitleArray = [];

    for (const sub of rawSubtitle) {
      subtitleArray.push({
        default: sub.disposition?.default === 1,
        forced: sub.disposition?.forced === 1,
        title: sub.tags?.title || "",
        language: sub.tags?.language || "und",
        isDeleted: false,
        isImported: false,
        path: null,
      });
    }

    this._oldSubtitle = subtitleArray;
    this._subtitle = structuredClone(subtitleArray);
  }

  reset() {
    this.initialized = false;
    this._oldVideo = {};
    this._oldAudio = [];
    this._oldSubtitle = [];

    this._video = {};
    this._audio = [];
    this._subtitle = [];
  }

  setVideo(prop, value = null) {
    switch (prop) {
      case "width":
        this._video.width = value;
        break;

      case "height":
        this._video.height = value;
        break;

      case "codec":
        this._video.codecName = value;
        break;

      case "aspectRatio":
        this._video.aspectRatio = value;
        break;

      case "pixelFormat":
        this._video.pixelFormat = value;
        break;

      case "profile":
        this._video.profile = value;
        break;

      case "fieldOrder":
        this._video.fieldOrder = value;
        break;
    }
  }

  getVideo() {
    return this._video;
  }

  get isVideo() {
    return Object.keys(this._video).length > 0 ? true : false;
  }

  setAudio(prop, index, value = null) {
    switch (prop) {
      case "default":
        if (this._audio[index].default) this._audio[index].default = false;
        else {
          this._audio.forEach((x) => (x.default = false));
          this._audio[index].default = true;
        }

        break;

      case "delete":
        this._audio[index].isDeleted = !this._audio[index].isDeleted;
        break;

      case "language":
        this._audio[index].language = value;
        break;

      case "codec":
        this._audio[index].codecName = value;
        break;

      case "bitrate":
        this._audio[index].bitRate = value.toString();
        break;

      case "samplerate":
        this._audio[index].sampleRate = value;
        break;

      case "channel":
        this._audio[index].newChannels = value;
        break;

      case "channelMap":
        this._audio[index].channelMap = value;
        break;

      case "channelVolume":
        this._audio[index].channelVolumes = value;
    }
  }

  addAudio(importPath, rawAudioStream) {
    let channelMap = {};
    let channelVolumes = {};
    let channels = rawAudioStream?.channels || 2;

    for (let i = 0; i < channels; i++) {
      channelMap[i] = i;
      channelVolumes[i] = 0;
    }

    this._audio.push({
      default: false,
      forced: false,
      title: "",
      language: "eng",
      isDeleted: false,
      isImported: true,
      path: importPath,
      bitRate: rawAudioStream?.bit_rate || null,
      bitDepth: rawAudioStream?.bits_per_sample || null,
      channels: channels,
      newChannels: channels,
      codecName: rawAudioStream?.codec_name || "copy",
      sampleRate: rawAudioStream?.sample_rate || null,
      channelMap,
      channelVolumes,
    });

    return this._audio.length - 1;
  }

  getAudioTracks() {
    return this._audio.map((x, idx) => ({
      code: idx,
      name: `${LANGUAGES.find((y) => y.code === x.language).name} [${AUDIO_CODECS[x.codecName]}]`,
    }));
  }

  getAudio(index) {
    return this._audio[index] ?? null;
  }

  get isAudio() {
    return this._audio.length > 0 ? true : false;
  }

  setSubtitle(prop, index, value = "") {
    switch (prop) {
      case "default":
        if (this._subtitle[index].default)
          this._subtitle[index].default = false;
        else {
          this._subtitle.forEach((x) => (x.default = false));
          this._subtitle[index].default = true;
        }

        break;

      case "forced":
        this._subtitle[index].forced = !this._subtitle[index].forced;
        break;

      case "delete":
        this._subtitle[index].isDeleted = true;
        break;

      case "title":
        this._subtitle[index].title = value;
        break;

      case "language":
        this._subtitle[index].language = value;
        break;
    }
  }

  addSubtitle(importPath) {
    this._subtitle.push({
      default: false,
      forced: false,
      title: "",
      language: "eng",
      isDeleted: false,
      isImported: true,
      path: importPath,
    });
    return this._subtitle.length - 1;
  }

  getSubtitles() {
    return this._subtitle;
  }

  get isSubtitle() {
    return this._subtitle.length > 0 ? true : false;
  }

  get isPendingChanges() {
    if (!this.initialized) return false;

    const vDiff =
      JSON.stringify(this._oldVideo) !== JSON.stringify(this._video);
    const aDiff =
      JSON.stringify(this._oldAudio) !== JSON.stringify(this._audio);
    const sDiff =
      JSON.stringify(this._oldSubtitle) !== JSON.stringify(this._subtitle);

    if (vDiff || aDiff || sDiff) return true;
    else return false;
  }

  getPendingChanges() {
    const vDiff =
      JSON.stringify(this._oldVideo) !== JSON.stringify(this._video);
    const aDiff =
      JSON.stringify(this._oldAudio) !== JSON.stringify(this._audio);
    const sDiff =
      JSON.stringify(this._oldSubtitle) !== JSON.stringify(this._subtitle);

    const getValidEncoder = (codec, type) => {
      if (!codec) return null;
      const lower = codec.toLowerCase();
      if (type === "video") {
        if (lower === "h264") return "libx264";
        if (lower === "hevc") return "libx265";
        if (lower === "vp8") return "libvpx";
        if (lower === "vp9") return "libvpx-vp9";
        if (lower === "av1") return "libsvtav1";
        if (lower === "prores") return "prores_ks";
      }
      if (type === "audio") {
        if (lower === "truehd" || lower === "dts" || lower === "dca")
          return "ac3";
      }
      return lower;
    };

    let videoPayload = null;
    if (vDiff && Object.keys(this._video).length > 0) {
      videoPayload = { ...this._video };

      const oldW =
        this._oldVideo.width && this._oldVideo.width !== ""
          ? Number(this._oldVideo.width)
          : null;
      const newW =
        this._video.width && this._video.width !== ""
          ? Number(this._video.width)
          : null;
      const oldH =
        this._oldVideo.height && this._oldVideo.height !== ""
          ? Number(this._oldVideo.height)
          : null;
      const newH =
        this._video.height && this._video.height !== ""
          ? Number(this._video.height)
          : null;

      const needsReencode =
        this._oldVideo.codecName !== this._video.codecName ||
        oldW !== newW ||
        oldH !== newH ||
        this._oldVideo.aspectRatio !== this._video.aspectRatio ||
        this._oldVideo.pixelFormat !== this._video.pixelFormat ||
        this._oldVideo.fieldOrder !== this._video.fieldOrder ||
        this._oldVideo.profile !== this._video.profile;

      if (!needsReencode) {
        videoPayload.codecName = "copy";
        videoPayload.width = null;
        videoPayload.height = null;
        videoPayload.aspectRatio = null;
        videoPayload.pixelFormat = null;
        videoPayload.fieldOrder = null;
        videoPayload.profile = null;
      } else {
        videoPayload.codecName = getValidEncoder(
          this._video.codecName,
          "video",
        );
        videoPayload.width = newW;
        videoPayload.height = newH;
      }
    }

    let audioPayload = null;
    if (aDiff && this._audio.length > 0) {
      audioPayload = this._audio.map((aud, i) => {
        const oldAud = this._oldAudio[i] || {};

        const needsReencode =
          aud.codecName !== oldAud.codecName ||
          String(aud.bitRate || "") !== String(oldAud.bitRate || "") ||
          String(aud.sampleRate || "") !== String(oldAud.sampleRate || "") ||
          Number(aud.newChannels || 0) !== Number(oldAud.newChannels || 0) ||
          JSON.stringify(aud.channelMap) !==
            JSON.stringify(oldAud.channelMap) ||
          JSON.stringify(aud.channelVolumes) !==
            JSON.stringify(oldAud.channelVolumes);

        const payload = { ...aud };
        if (!needsReencode) {
          payload.codecName = "copy";
          payload.bitRate = null;
          payload.sampleRate = null;
          payload.newChannels = null;
          payload.channelMap = null;
          payload.channelVolumes = null;
        } else {
          payload.codecName = getValidEncoder(aud.codecName, "audio");
          payload.newChannels =
            payload.newChannels && payload.newChannels !== ""
              ? Number(payload.newChannels)
              : null;
          payload.bitRate =
            payload.bitRate && payload.bitRate !== ""
              ? String(payload.bitRate)
              : null;
          payload.sampleRate =
            payload.sampleRate && payload.sampleRate !== ""
              ? String(payload.sampleRate)
              : null;
        }
        return payload;
      });
    }

    return {
      video: videoPayload,
      audio: audioPayload,
      subtitle: sDiff ? this._subtitle : null,
    };
  }
}

export class MediaDataComparer {
  mediaDataFiles = $state([]);
  initialized = $state(false);

  targetProfile = $state({
    video: {
      width: null,
      height: null,
      codecName: null,
      aspectRatio: null,
      pixelFormat: null,
      fieldOrder: null,
      profile: null,
    },
    audio: {
      forceCodec: null,
      forceSampleRate: null,
      forceChannels: null,
      forceBitrate: null,
      setDefaultStream: null,
      setForcedStream: null,
      keepOnlyStream: null,
      deleteStream: null,
      deleteAll: false,
    },
    subtitle: {
      setDefaultStream: null,
      setForcedStream: null,
      keepOnlyStream: null,
      deleteStream: null,
      deleteAll: false,
    },
  });

  addMediaData(data) {
    const tempMediaData = new MediaData();
    tempMediaData.path = data.path;
    tempMediaData.init(data);
    this.mediaDataFiles.push(tempMediaData);
  }

  getUniqueValues(category, property) {
    if (!property) return [];
    const values = new Set();

    for (const file of this.mediaDataFiles) {
      if (category === "video" && file._video[property]) {
        values.add(file._video[property]);
      } else if (category === "audio") {
        for (const stream of file._audio) {
          if (property === "fullTrack") {
            const langCode = stream.language || "und";
            const langName =
              LANGUAGES.find((x) => x.code === langCode)?.name || langCode;
            const title = stream.title ? ` [${stream.title}]` : "";
            values.add(langName + title);
          } else if (
            stream[property] !== undefined &&
            stream[property] !== null
          ) {
            values.add(stream[property]);
          }
        }
      } else if (category === "subtitle") {
        for (const stream of file._subtitle) {
          if (property === "fullTrack") {
            const langCode = stream.language || "und";
            const langName =
              LANGUAGES.find((x) => x.code === langCode)?.name || langCode;
            const title = stream.title ? ` [${stream.title}]` : "";
            values.add(langName + title);
          } else if (
            stream[property] !== undefined &&
            stream[property] !== null
          ) {
            values.add(stream[property]);
          }
        }
      }
    }
    return Array.from(values);
  }

  setTarget(category, property, value) {
    this.targetProfile[category][property] = value;
  }

  getTarget(category, property) {
    return this.targetProfile[category][property];
  }

  exportProfile() {
    return JSON.stringify(this.targetProfile, null, 2);
  }

  importProfile(jsonString) {
    try {
      const parsed = JSON.parse(jsonString);
      if (parsed.video && parsed.audio && parsed.subtitle) {
        this.targetProfile = parsed;
      }
    } catch (e) {
      console.error(e);
    }
  }

  reset() {
    this.mediaDataFiles = [];
    this.initialized = false;
    this.targetProfile = {
      video: {
        width: null,
        height: null,
        codecName: null,
        aspectRatio: null,
        pixelFormat: null,
        fieldOrder: null,
        profile: null,
      },
      audio: {
        forceCodec: null,
        forceSampleRate: null,
        forceChannels: null,
        forceBitrate: null,
        setDefaultStream: null,
        setForcedStream: null,
        keepOnlyStream: null,
        deleteStream: null,
        deleteAll: false,
      },
      subtitle: {
        setDefaultStream: null,
        setForcedStream: null,
        keepOnlyStream: null,
        deleteStream: null,
        deleteAll: false,
      },
    };
  }

  applyRules() {
    const profile = this.targetProfile;

    for (const file of this.mediaDataFiles) {
      if (file.isVideo) {
        if (profile.video.width) file.setVideo("width", profile.video.width);
        if (profile.video.height) file.setVideo("height", profile.video.height);
        if (profile.video.codecName)
          file.setVideo("codec", profile.video.codecName);
        if (profile.video.aspectRatio)
          file.setVideo("aspectRatio", profile.video.aspectRatio);
        if (profile.video.pixelFormat)
          file.setVideo("pixelFormat", profile.video.pixelFormat);
        if (profile.video.fieldOrder)
          file.setVideo("fieldOrder", profile.video.fieldOrder);
        if (profile.video.profile)
          file.setVideo("profile", profile.video.profile);
      }

      const getStreamName = (s) => {
        const langCode = s.language || "und";
        const langName =
          LANGUAGES.find((x) => x.code === langCode)?.name || langCode;
        return langName + (s.title ? ` [${s.title}]` : "");
      };

      const applyStreamRules = (type) => {
        const streamArray = type === "audio" ? file._audio : file._subtitle;
        const setMethod = type === "audio" ? "setAudio" : "setSubtitle";
        const rules = profile[type];

        if (rules.deleteAll) {
          streamArray.forEach((_, i) => file[setMethod]("delete", i));
        } else {
          streamArray.forEach((stream, i) => {
            if (type === "audio") {
              if (rules.forceCodec)
                file[setMethod]("codec", i, rules.forceCodec);
              if (rules.forceSampleRate)
                file[setMethod]("samplerate", i, rules.forceSampleRate);
              if (rules.forceChannels)
                file[setMethod]("channel", i, rules.forceChannels);
              if (rules.forceBitrate)
                file[setMethod]("bitrate", i, rules.forceBitrate);
            }
            const streamName = getStreamName(stream);
            if (rules.deleteStream && streamName === rules.deleteStream)
              file[setMethod]("delete", i);
            if (rules.keepOnlyStream && streamName !== rules.keepOnlyStream)
              file[setMethod]("delete", i);
          });

          const setFlag = (targetStreamName, flagName) => {
            if (targetStreamName) {
              const targetIdx = streamArray.findIndex(
                (s) => !s.isDeleted && getStreamName(s) === targetStreamName,
              );
              if (targetIdx !== -1) {
                if (flagName === "default")
                  streamArray.forEach(
                    (_, i) => (streamArray[i].default = false),
                  );
                streamArray[targetIdx][flagName] = true;
              }
            }
          };

          setFlag(rules.setDefaultStream, "default");
          setFlag(rules.setForcedStream, "forced");
        }
      };

      if (file.isAudio) applyStreamRules("audio");
      if (file.isSubtitle) applyStreamRules("subtitle");
    }
  }

  getBatchPayload() {
    this.applyRules();
    let payload = [];
    for (const file of this.mediaDataFiles) {
      if (file.isPendingChanges) {
        payload.push({
          filePath: file.path,
          changes: file.getPendingChanges(),
        });
      }
    }
    return payload;
  }
}
