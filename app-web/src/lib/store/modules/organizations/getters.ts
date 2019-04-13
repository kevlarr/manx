import {
  Org,
  OrgState,
  Stream,
  OrgsState as State
} from '@/types';

export default {
  /**
   * Return list of all organizations sorted by title
   */
  all(s: State): Org[] {
    return Object.values(s)
      .map((orgState: OrgState) => orgState.org)
      .sort((a: Org, b: Org) => a.title > b.title ? 1 : 0);
  },

  /**
   * Returns a closure that can retrieve organization from state by `shortId`
   */
  byShortId(s: State): (arg: string) => (Org | undefined) {
    return (shortId: string) => Object.values(s)
      .map((orgState: OrgState) => orgState.org)
      .find((org: Org) => org.shortId === shortId);
  },

  /**
   * Returns a list of all streams for the given organization `shortId`
   */
  streamsFor(s: State): (arg: string) => Stream[] {
    return (shortId: string) => {
      const orgState = Object.values(s)
        .find((orgState: OrgState) => orgState.org.shortId === shortId)!;

      return Object.values(orgState.streams);
    };
  },

  /**
   * Returns the global stream for an org
   */
  globalStream(s: State): (arg: string) => Stream {
    return (orgShortId: string) => {
      const orgState = Object.values(s)
        .find((orgState: OrgState) => orgState.org.shortId === orgShortId)!;

      return Object.values(orgState.streams)
        .find((s: Stream) => s.global)!;
    };
  },

  /**
   * Finds the stream by given org/stream shortIds
   */
  streamByShortId(s: State): (arg0: string, arg1: string) => (Stream | undefined) {
    return (orgShortId: string, streamShortId: string) => {
      const orgState = Object.values(s)
        .find((os: OrgState) => os.org.shortId === orgShortId)!;

      return Object.values(orgState.streams)
        .find((stream: Stream) => stream.shortId === streamShortId);
    };
  },
};
