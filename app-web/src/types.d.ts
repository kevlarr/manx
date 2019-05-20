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

interface Organization {
  id: number;
  creatorId: number;
  shortId: string;
  title: string;
}

interface NewOrganization {
  title: string;
  userName: string;
}

interface OrganizationUser {
  id: number;
  organizationId: number;
  name: string;
}

interface Stream {
  id: number;
  parentId: number | null;
  organizationId: number;
  global: boolean;
  name: string;
  shortId: string;
}

interface NewStream {
  organizationId: string;
  name: string;
}

interface Topic {
  id: number;
  streamId: number;
  raw: string;
  rendered: string;
}

interface NewTopic {
  streamId: number;
  raw: string;
}

interface Comment {
  // TODO
}

interface NewComment {
  // TODO
}
