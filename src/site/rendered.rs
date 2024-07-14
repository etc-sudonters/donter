use url::form_urlencoded::Target;

use super::IncludedAsset;
use super::SiteError;
use super::Writable;
use crate::content;
use crate::content::CorpusEntry;
use crate::files;
use crate::ids;
use crate::jinja;
use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::DerefMut;

pub struct RenderingSite<'rendering, 'site, 'env>
where
    'env: 'site,
    'site: 'rendering,
{
    renderer: jinja::Renderer<'rendering, 'env>,
    ids: ids::IdPool<RenderedSite<'site>>,
    site: RenderedSite<'site>,
}

impl<'rendering, 'site, 'env> RenderingSite<'rendering, 'site, 'env>
where
    'env: 'site,
    'site: 'rendering,
{
    pub fn new(renderer: jinja::Renderer<'rendering, 'env>) -> Self {
        Self {
            ids: ids::IdPool::new(1),
            site: RenderedSite::new(),
            renderer,
        }
    }

    pub fn render(self) -> RenderedSite<'site> {
        self.site
    }

    pub fn page<'page>(&mut self, template: &'page str) -> RenderingPage<'page, 'site> {
        RenderingPage {
            id: self.ids.next(),
            tpl: template,
            v: jinja::RenderContext::empty(),
        }
    }

    pub fn render_page<'page, M: Into<RenderedPageMetadata<'site>>>(
        &'page mut self,
        meta: M,
        page: RenderingPage<'page, 'site>,
    ) -> crate::Result<()>
    where
        'site: 'page,
    {
        let rendered = self.renderer.render_template(&page.tpl, page.v)?;
        let page = RenderedPage {
            id: page.id,
            content: VecDeque::from(rendered.into_bytes()),
            meta: meta.into(),
        };
        if let Some(ref origin) = page.meta.origin {
            self.site.origins.insert(origin.clone(), page.id.clone());
        }
        self.site
            .writables
            .insert(page.id.clone(), Writable::Page(page));
        Ok(())
    }

    pub fn add_asset(&mut self, asset: IncludedAsset) {
        self.site
            .writables
            .insert(self.ids.next(), Writable::Asset(asset));
    }

    pub fn get_by_origin<K>(&self, origin: K) -> Option<&Writable<'site>>
    where
        K: std::borrow::Borrow<ids::Id<CorpusEntry>>,
    {
        match self.site.origins.get(origin.borrow()) {
            None => None,
            Some(dest) => self.site.writables.get(dest), // this will always be Some(...)
        }
    }
}

pub struct RenderingPage<'page, 'site>
where
    'site: 'page,
{
    id: ids::Id<RenderedSite<'site>>,
    tpl: &'page str,
    v: jinja::RenderContext,
}

impl<'page, 'site> RenderingPage<'page, 'site>
where
    'site: 'page,
{
    pub fn values(&mut self) -> &mut jinja::RenderContext {
        &mut self.v
    }
}

pub struct RenderedSite<'site> {
    writables: HashMap<ids::Id<RenderedSite<'site>>, Writable<'site>>,
    origins: HashMap<ids::Id<CorpusEntry>, ids::Id<RenderedSite<'site>>>,
}

pub struct RenderedPage<'site> {
    id: ids::Id<RenderedSite<'site>>,
    content: VecDeque<u8>,
    meta: RenderedPageMetadata<'site>,
}

pub struct PageTemplate<'a> {
    pub(crate) title: &'a str,
    pub(crate) url: files::FilePath,
    pub(crate) template: &'a str,
}

#[derive(Debug, Clone)]
pub struct RenderedPageMetadata<'site> {
    pub(crate) origin: Option<ids::Id<CorpusEntry>>,
    pub(crate) title: Cow<'site, str>,
    pub(crate) url: Cow<'site, files::FilePath>,
    pub(crate) when: Option<Cow<'site, str>>,
    pub(crate) summary: Option<String>,
}

impl<'a> PageTemplate<'a> {
    pub fn borrow<'b>(&'b self) -> RenderedPageMetadata<'b>
    where
        'a: 'b,
    {
        RenderedPageMetadata {
            title: Cow::Borrowed(&self.title),
            url: Cow::Borrowed(&self.url),
            when: None,
            summary: None,
            origin: None,
        }
    }

    pub fn stamp<'b>(&self) -> RenderedPageMetadata<'b> {
        RenderedPageMetadata {
            title: Cow::Owned(self.title.to_owned()),
            url: Cow::Owned(self.url.clone()),
            when: None,
            summary: None,
            origin: None,
        }
    }
}

impl<'site> RenderedPage<'site> {
    pub fn read(self) -> impl std::io::Read {
        self.content
    }

    pub fn size(&self) -> u64 {
        self.content.len() as u64
    }

    pub fn metadata(&self) -> &RenderedPageMetadata {
        &self.meta
    }
}

impl<'env> RenderedSite<'env> {
    pub fn new() -> Self {
        Self {
            writables: Default::default(),
            origins: Default::default(),
        }
    }

    pub fn entries(
        self,
    ) -> impl std::iter::Iterator<Item = (ids::Id<RenderedSite<'env>>, Writable<'env>)> {
        self.writables.into_iter()
    }

    pub fn get<K>(&self, id: K) -> Option<&Writable<'env>>
    where
        K: std::borrow::Borrow<ids::Id<RenderedSite<'env>>>,
    {
        self.writables.get(id.borrow())
    }
}
