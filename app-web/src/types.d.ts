import Vue from 'vue';
import { NavigationGuard } from 'vue-router';
import Repo from '@/lib/repo';

/**
 * Vue
 */

type Next = Parameters<NavigationGuard>[2];

/**
 * Data models
 */

interface Team {
  id: number;
  created: Date;
  title: string;
  key: string;
}

interface NewTeam {
  title: string;
  username: string;
}

interface Member {
  id: number;
  teamId: number;
  created: Date;
  name: string;
}

interface Stream {
  id: number;
  teamId: number;
  memberId: number;
  parentId: number | null;
  created: Date;
  title: string;
  key: string;
}

interface NewStream {
  parentId: number | null;
  title: string;
}

interface Topic {
  id: number;
  memberId: number;
  streamId: number;
  created: Date;
  updated: Date;
  raw: string;
  rendered: string;
  key: string;
}

interface NewTopic {
  raw: string;
}

interface Comment {
  id: number;
  memberId: number;
  topicId: number;
  created: Date;
  updated: Date;
  raw: string;
  rendered: string;
}

interface NewComment {
  raw: string;
}
