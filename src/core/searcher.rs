use Result;
use core::SegmentReader;
use schema::Document;
use collector::Collector;
use common::TimerTree;
use query::Query;
use DocId;
use DocAddress;
use schema::Term;
use core::TermIterator;


/// Holds a list of `SegmentReader`s ready for search.
///
/// It guarantees that the `Segment` will not be removed before  
/// the destruction of the `Searcher`.
/// 
#[derive(Debug)]
pub struct Searcher {
    segment_readers: Vec<SegmentReader>,
}

impl Searcher {
      
    /// Fetches a document from tantivy's store given a `DocAddress`.
    ///
    /// The searcher uses the segment ordinal to route the
    /// the request to the right `Segment`. 
    pub fn doc(&self, doc_address: &DocAddress) -> Result<Document> {
        let DocAddress(segment_local_id, doc_id) = *doc_address;
        let segment_reader = &self.segment_readers[segment_local_id as usize];
        segment_reader.doc(doc_id)
    }
    
    /// Returns the overall number of documents in the index.
    pub fn num_docs(&self,) -> DocId {
        self.segment_readers
            .iter()
            .map(|segment_reader| segment_reader.num_docs())
            .fold(0u32, |acc, val| acc + val)
    }
    
    /// Return the overall number of documents containing
    /// the given term. 
    pub fn doc_freq(&self, term: &Term) -> u32 {
        self.segment_readers
            .iter()
            .map(|segment_reader| segment_reader.doc_freq(term))
            .fold(0u32, |acc, val| acc + val)
    }

    /// Returns a Stream over all of the sorted unique terms of
    /// the searcher.
    ///
    /// This includes all of the fields from all of the segment_readers.
    /// See [TermIterator](struct.TermIterator.html).
    ///
    /// # Warning
    /// This API is very likely to change in the future.
    pub fn terms<'a>(&'a self) -> TermIterator<'a> {
        TermIterator::from(self.segment_readers())
    }

    /// Return the list of segment readers
    pub fn segment_readers(&self,) -> &[SegmentReader] {
        &self.segment_readers
    }
    
    /// Returns the segment_reader associated with the given segment_ordinal
    pub fn segment_reader(&self, segment_ord: usize) -> &SegmentReader {
        &self.segment_readers[segment_ord]
    }
       
    /// Runs a query on the segment readers wrapped by the searcher
    pub fn search<C: Collector>(&self, query: &Query, collector: &mut C) -> Result<TimerTree> {
        query.search(self, collector)
    }
}

impl From<Vec<SegmentReader>> for Searcher {
    fn from(segment_readers: Vec<SegmentReader>) -> Searcher {
        Searcher {
            segment_readers: segment_readers,
        }
    }
}